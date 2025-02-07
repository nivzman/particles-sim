use femtovg::Color;

pub type Vector = euclid::default::Vector2D<f32>;
pub type Point = euclid::default::Point2D<f32>;

pub const WORLD_WIDTH: u32 = 1000;
pub const WORLD_HEIGHT: u32 = 1000;
pub const WORLD_WIDTH_FLOAT: f32 = WORLD_WIDTH as f32;
pub const WORLD_HEIGHT_FLOAT: f32 = WORLD_HEIGHT as f32;
pub const PARTICLE_RADIUS: f32 = 3.;
pub const WORLD_WIDTH_BOUND: f32 = WORLD_WIDTH_FLOAT - PARTICLE_RADIUS;
pub const WORLD_HEIGHT_BOUND: f32 = WORLD_HEIGHT_FLOAT - PARTICLE_RADIUS;


pub struct Particle {
    pub(super) position: Point,
    pub(super) velocity: Vector,
    pub(super) color: ParticleColor,
}

#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParticleColor {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl ParticleColor {
    pub const fn matrix_len() -> usize {
        return (ParticleColor::Blue as usize) + 1;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum WorldEdge {
    Left,
    Right,
    Bottom,
    Top,
}
impl Into<Color> for ParticleColor {
    fn into(self) -> Color {
        match self {
            ParticleColor::Blue => Color::rgb(0, 0, 255),
            ParticleColor::Red => Color::rgb(255, 0, 0),
            ParticleColor::Green => Color::rgb(0, 255, 0),
        }
    }
}

impl Particle {
    pub fn new(position: Point, velocity: Vector, color: ParticleColor) -> Self {
        Particle {
            position,
            velocity,
            color
        }
    }
}

pub struct ForceConfig {
    matrix: [[f32; ParticleColor::matrix_len()]; ParticleColor::matrix_len()]
}

impl ForceConfig {
    pub fn empty() -> Self {
        ForceConfig {
            matrix: [[0.; ParticleColor::matrix_len()]; ParticleColor::matrix_len()]
        }
    }

    pub fn with_force(mut self, who: ParticleColor, to: ParticleColor, force: f32) -> Self {
        self.matrix[who as usize][to as usize] = force;
        self
    }

    pub fn get(&self, who: ParticleColor, to: ParticleColor) -> f32 {
        self.matrix[who as usize][to as usize]
    }
}