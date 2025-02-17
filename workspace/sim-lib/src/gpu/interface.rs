use cust_core::DeviceCopy;

#[derive(Copy, Clone, DeviceCopy)]
#[repr(u32)]
pub enum ParticleColor {
    Red = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
}

pub const NUM_COLORS: usize = 4;

#[derive(Clone, Copy, DeviceCopy)]
#[repr(C, packed(1))]
pub struct Constants {
    pub world_unit_size: f32,
    pub force_scaler: f32,
    pub repel_force_radius: f32,
}

#[derive(Clone, Copy, DeviceCopy)]
#[repr(C, packed(1))]
pub struct Particle {
    pub position_x: f32,
    pub position_y: f32,
    pub color: ParticleColor,
}

#[derive(Clone, Copy, DeviceCopy)]
#[repr(C, packed(1))]
pub struct OutcomeAcceleration {
    pub acceleration_x: f32,
    pub acceleration_y: f32,
}
