use std::collections::HashMap;
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


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParticleColor {
    Red,
    Green,
    Blue,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum WorldEdge {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ForceRelation {
    pub who: ParticleColor,
    pub to: ParticleColor,
}

pub type ForcesConfiguration = HashMap<ForceRelation, f32>;

impl Into<Color> for ParticleColor {
    fn into(self) -> Color {
        match self {
            ParticleColor::Blue => Color::rgb(0, 0, 255),
            ParticleColor::Red => Color::rgb(255, 0, 0),
            ParticleColor::Green => Color::rgb(0, 255, 0),
        }
    }
}

