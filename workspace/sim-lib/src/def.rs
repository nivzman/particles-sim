use femtovg::Color;
use rand::Rng;

pub type Vector = euclid::default::Vector2D<f32>;
pub type Point = euclid::default::Point2D<f32>;

#[derive(Copy, Clone)]
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
            ParticleColor::Yellow => Color::rgb(252, 186, 3),
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

#[derive(Copy, Clone)]
pub struct ForcesConfig {
    matrix: [[f32; ParticleColor::matrix_len()]; ParticleColor::matrix_len()]
}

impl ForcesConfig {
    pub fn empty() -> Self {
        ForcesConfig {
            matrix: [[0.; ParticleColor::matrix_len()]; ParticleColor::matrix_len()]
        }
    }

    pub fn random(min_force: f32, max_force: f32) -> Self {
        let max_force = f32::abs(max_force);
        let mut rng = rand::rng();
        (0..ParticleColor::matrix_len()*ParticleColor::matrix_len()).fold(Self::empty(), |c, i| {
            c.with_force_unchecked(i / ParticleColor::matrix_len(), i % ParticleColor::matrix_len(), rng.random_range(min_force..max_force))
        })
    }

    pub fn with_force(self, who: ParticleColor, to: ParticleColor, force: f32) -> Self {
        self.with_force_unchecked(who as usize, to as usize, force)
    }

    pub fn get(&self, who: ParticleColor, to: ParticleColor) -> f32 {
        self.matrix[who as usize][to as usize]
    }

    fn with_force_unchecked(mut self, who: usize, to: usize, force: f32) -> Self {
        self.matrix[who][to] = force;
        self
    }
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
        Yellow = 3
    }
}

impl ParticleColor {
    const fn matrix_len() -> usize {
        return (ParticleColor::max_value()) + 1;
    }
}
