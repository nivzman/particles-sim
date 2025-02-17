use cust::memory::DeviceBox;
use cust::prelude::*;
use crate::{ForcesConfig, Particle, Vector, ParticleColor as CrateParticleColor};
use interface::ParticleColor as InterfaceParticleColor;

mod interface;

static PTX: &str = include_str!("../../resources/gpu_kernel.ptx");
static PTX_KERNEL_NAME: &str = "calculate_emergence_acceleration";

const FORCES_CONFIG_ARRAY_LEN: usize = interface::NUM_COLORS * interface::NUM_COLORS;
const _: () = assert!(interface::NUM_COLORS == CrateParticleColor::matrix_len());

const ZERO_ACCELERATION: interface::OutcomeAcceleration = interface::OutcomeAcceleration {
    acceleration_x: 0.0,
    acceleration_y: 0.0,
};

pub struct Executor {
    _context: Context,
    module: Module,
    stream: Stream,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            _context: cust::quick_init().unwrap(),
            module: Module::from_ptx(PTX, &[]).unwrap(),
            stream: Stream::new(StreamFlags::NON_BLOCKING, None).unwrap(),
        }
    }

    pub(crate) fn calculate_emergence_accelerations(&self, particles: &[Particle], forces_config: &ForcesConfig) -> Vec<Vector> {
        let mut outcomes = vec![ZERO_ACCELERATION; particles.len()];
        let outcomes_gpu = outcomes.as_slice().as_dbuf().unwrap();
        let constants_gpu = Self::alloc_gpu_constants();
        let forces_gpu = Self::alloc_gpu_forces_config(&forces_config);
        let particles_gpu = Self::alloc_particles_gpu(particles);

        let func = self.module.get_function(PTX_KERNEL_NAME).unwrap();
        let (_, block_size) = func.suggested_launch_configuration(0, 0.into()).unwrap();
        let grid_size = (particles.len() as u32 + block_size - 1) / block_size;

        let stream = &self.stream;

        unsafe {
            launch!(
                func<<<grid_size, block_size, 0, stream>>>(
                    particles_gpu.as_device_ptr(),
                    particles_gpu.len(),
                    constants_gpu.as_device_ptr(),
                    forces_gpu.as_device_ptr(),
                    forces_gpu.len(),
                    outcomes_gpu.as_device_ptr(),
                )
            ).unwrap();
        }

        stream.synchronize().unwrap();
        outcomes_gpu.copy_to(&mut outcomes).unwrap();

        outcomes.iter().map(|acc_gpu| Vector::new(acc_gpu.acceleration_x, acc_gpu.acceleration_y)).collect()
    }

    fn alloc_gpu_constants() -> DeviceBox<interface::Constants> {
        let c = interface::Constants {
            world_unit_size: crate::constants::WORLD_UNIT_SIZE,
            force_scaler: crate::constants::FORCE_SCALAR,
            repel_force_radius: crate::physics::emergence::GLOBAL_REPEL_FORCE_RADIUS,
        };

        c.as_dbox().unwrap()
    }

    fn alloc_gpu_forces_config(forces_config: &ForcesConfig) -> DeviceBuffer<f32> {
        let mut arr = [0.0f32; FORCES_CONFIG_ARRAY_LEN];
        (0..FORCES_CONFIG_ARRAY_LEN).for_each(|i| {
            arr[i] = forces_config.get_unchecked(i / interface::NUM_COLORS, i % interface::NUM_COLORS);
        });
        arr.as_slice().as_dbuf().unwrap()
    }

    fn alloc_particles_gpu(particles: &[Particle]) -> DeviceBuffer<interface::Particle> {
        let mut v = Vec::with_capacity(particles.len());
        particles.iter().for_each(|p| {
            let p_interface = interface::Particle {
                position_x: p.position.x,
                position_y: p.position.y,
                color: p.color.into(),
            };
            v.push(p_interface);
        });
        v.as_slice().as_dbuf().unwrap()
    }
}

impl Into<InterfaceParticleColor> for CrateParticleColor {
    fn into(self) -> InterfaceParticleColor {
        match self {
            CrateParticleColor::Blue => InterfaceParticleColor::Blue,
            CrateParticleColor::Red => InterfaceParticleColor::Red,
            CrateParticleColor::Green => InterfaceParticleColor::Green,
            CrateParticleColor::Yellow => InterfaceParticleColor::Yellow,
        }
    }
}
