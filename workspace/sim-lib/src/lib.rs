mod calc;
mod def;
mod physics;
mod constants;

use std::sync::Arc;
use crossbeam_channel::Sender;
use femtovg::Canvas;

pub use threadpool::ThreadPool;
pub use def::{Particle, ParticleColor, Point, Vector, ForcesConfig};
pub use physics::PhysicsMode;
pub use calc::{random_world_position, bounded_value};

pub struct Simulation {
    particles: Vec<Particle>,
    forces: ForcesConfig,
    physics_mode: PhysicsMode,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>, forces: ForcesConfig, physics: PhysicsMode) -> Self {
        Simulation {
            particles,
            forces,
            physics_mode: physics,
        }
    }

    pub fn tick(&mut self, thread_pool: Option<&ThreadPool>) {
        self.update_velocities(thread_pool);
        self.update_positions();
    }

    pub fn draw<R: femtovg::Renderer>(&self, canvas: &mut Canvas<R>, camera_position: Point, scale_factor: f32) {
        let min_x = camera_position.x;
        let max_x = camera_position.x + (canvas.width() as f32 / scale_factor);
        let min_y = camera_position.y;
        let max_y = camera_position.y + (canvas.height() as f32 / scale_factor);

        for particle in self.particles.iter() {
            if particle.position.x < min_x || particle.position.x > max_x || particle.position.y < min_y || particle.position.y > max_y {
                continue;
            }

            let mut path = femtovg::Path::new();
            path.circle((particle.position.x - camera_position.x) * scale_factor, (particle.position.y - camera_position.y) * scale_factor, constants::BASE_PARTICLE_RADIUS * scale_factor);
            canvas.fill_path(&path, &femtovg::Paint::color(particle.color.into()));
        }
    }

    pub fn set_forces_config(&mut self, forces: ForcesConfig) {
        self.forces = forces;
    }

    pub fn get_forces_config(&self) -> ForcesConfig {
        self.forces
    }

    pub fn accelerate_all(&mut self, amount: f32) {
        self.particles.iter_mut().for_each(|p| p.velocity = p.velocity.with_length(p.velocity.length() + f32::abs(amount)));
    }

    fn update_velocities(&mut self, thread_pool: Option<&ThreadPool>) {
        if self.physics_mode == PhysicsMode::Emergence {
            self.particles.iter_mut().for_each(physics::emergence::apply_friction);
        }

        match thread_pool {
            Some(pool) => self.thread_pool_update_velocities(pool),
            None => self.no_thread_pool_update_velocities(),
        }
    }

    fn update_positions(&mut self) {
        self.particles.iter_mut().for_each(|particle| {
            particle.position += particle.velocity;
            match self.physics_mode {
                PhysicsMode::Real => physics::real::out_of_bounds_fixup(particle),
                PhysicsMode::Emergence  => physics::emergence::out_of_bounds_fixup(particle),
            }
        })
    }

    fn no_thread_pool_update_velocities(&mut self) {
        for i in 0..self.particles.len() {
            for j in i + 1..self.particles.len() {
                self.update_velocity_for(i, j);
                self.update_velocity_for(j, i);
            }
        }
    }

    fn thread_pool_update_velocities(&mut self, thread_pool: &ThreadPool) {
        let total_jobs = thread_pool.max_count();
        let (sender, receiver) = crossbeam_channel::bounded(total_jobs);
        let copied_particles = Arc::new(self.particles.clone());
        let common_job_chunk_size = copied_particles.len() / total_jobs;
        let last_job_chunk_size = common_job_chunk_size + (copied_particles.len() % total_jobs);

        (0..total_jobs).for_each(|job_index| {
            let chunk_size = if job_index == total_jobs - 1 {last_job_chunk_size} else { common_job_chunk_size };
            let chunk_start_index = job_index*common_job_chunk_size;
            Self::start_accelerations_calculation_job(
                copied_particles.clone(),
                chunk_start_index,
                chunk_size,
                thread_pool,
                sender.clone(),
                self.forces,
                self.physics_mode
            )
        });

        receiver.iter().take(total_jobs).for_each(|job_result| {
            job_result.accelerations.into_iter().enumerate().for_each(|(i, a)| {
                if let Some(a) = a {
                    self.particles[job_result.chunk_start_index + i].velocity += a;
                }
            })
        });
    }

    fn start_accelerations_calculation_job(
        particles: Arc<Vec<Particle>>,
        chunk_start_index: usize,
        chunk_size: usize,
        thread_pool: &ThreadPool,
        result_sender: Sender<JobResult>,
        forces: ForcesConfig,
        physics_mode: PhysicsMode)
    {
        thread_pool.execute(move || {
            let mut result = JobResult {
                chunk_start_index,
                accelerations: vec![None; chunk_size],
            };

            for i in 0..chunk_size {
                for j in 0..particles.len() {
                    let acc = calc::acceleration_of(&particles[chunk_start_index+i], &particles[j], &forces, physics_mode);
                    result.accelerations[i] = result.accelerations[i].map_or(acc, |x| Some(acc.map_or(x, |y| x + y)));
                }
            }

            result_sender.send(result).expect("Results channel will be there waiting for the pool");
        });
    }

    fn update_velocity_for(&mut self, target_particle_index: usize, other_particle_index: usize) {
        if let Some(acc) = calc::acceleration_of(
            &self.particles[target_particle_index],
            &self.particles[other_particle_index],
            &self.forces,
            self.physics_mode
        ) {
            self.particles[target_particle_index].velocity += acc;
        }
    }
}

struct JobResult {
    chunk_start_index: usize,
    accelerations: Vec<Option<Vector>>,
}
