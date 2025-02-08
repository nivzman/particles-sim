use femtovg::Color;

pub type Vector = euclid::default::Vector2D<f32>;
pub type Point = euclid::default::Point2D<f32>;

pub struct Particle {
    pub(crate) position: Point,
    pub(crate) velocity: Vector,
    pub(crate) color: ParticleColor,
}

#[derive(Eq, PartialEq)]
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
            ParticleColor::Green => Color::rgb(0, 255, 0),
            ParticleColor::Red => Color::rgb(255, 0, 0),
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

pub struct ForcesConfig {
    matrix: [[f32; ParticleColor::matrix_len()]; ParticleColor::matrix_len()]
}

impl ForcesConfig {
    pub fn empty() -> Self {
        ForcesConfig {
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

pub enum CameraMoveRequest {
    Right,
    Left,
    Up,
    Down,
}

pub enum CameraZoomRequest {
    In,
    Out,
}

macro_rules! define_particle_color {
    (
        $name:ident {
            $($variant:ident = $val:expr),+ $(,)?
        }
    ) => {
        #[derive(Copy, Clone)]
        #[repr(usize)]
        pub enum $name {
            $($variant = $val),+
        }

        impl $name {
            #![allow(unused_comparisons)]
            pub const fn max_value() -> usize {
                let mut max = 0;
                $(
                    max = if $val > max { $val } else { max };
                )+
                max
            }
        }
    };
}

define_particle_color! {
    ParticleColor {
        Red = 0,
        Green = 1,
        Blue = 2,
    }
}

impl ParticleColor {
    const fn matrix_len() -> usize {
        return (ParticleColor::max_value()) + 1;
    }
}
