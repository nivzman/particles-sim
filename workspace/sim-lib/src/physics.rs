use crate::{
    calc, Particle, random_world_position,
    constants::{WORLD_HEIGHT_FLOAT, WORLD_WIDTH_FLOAT},
    def::WorldEdge,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PhysicsMode {
    Real,
    Emergence,
}

pub mod real {
    use super::*;

    pub const MAX_APPLIED_FORCE: f32 = 0.1;

    pub fn calculate_force(configured_force: f32, distance: f32) -> f32 {
        if distance == 0. {
            return 0.;
        }

        let force = configured_force / distance.powi(2);
        calc::bounded_value(force, -MAX_APPLIED_FORCE, MAX_APPLIED_FORCE)
    }

    pub fn out_of_bounds_fixup(particle: &mut Particle) {
        let Some(edge) = calc::check_out_of_bounds(&particle.position) else {
            return;
        };

        match edge {
            WorldEdge::Left | WorldEdge::Right => {
                particle.velocity.x = -particle.velocity.x;
                if edge == WorldEdge::Right {
                    particle.position.x = WORLD_WIDTH_FLOAT
                } else {
                    particle.position.x = 0.
                }
            },
            WorldEdge::Top | WorldEdge::Bottom => {
                particle.velocity.y = -particle.velocity.y;
                if edge == WorldEdge::Bottom {
                    particle.position.y = WORLD_HEIGHT_FLOAT
                } else {
                    particle.position.y = 0.
                }
            }
        }
    }
}

pub mod emergence {
    use super::*;

    pub const FRICTION_MULTIPLIER: f32 = 0.65;
    pub const GLOBAL_REPEL_FORCE_RADIUS: f32 = 0.35;

    pub fn out_of_bounds_fixup(particle: &mut Particle) {
        if !calc::is_out_of_bounds(&particle.position) {
            return;
        }

        particle.position = random_world_position();
    }

    pub fn calculate_force(configured_force: f32, distance: f32) -> f32 {
        if distance <= GLOBAL_REPEL_FORCE_RADIUS {
            (distance / GLOBAL_REPEL_FORCE_RADIUS) - 1.
        } else if distance < 1. {
            let numerator = f32::abs((2. * distance) - 1. - GLOBAL_REPEL_FORCE_RADIUS);
            let denominator = 1. - GLOBAL_REPEL_FORCE_RADIUS;
            configured_force * (1. - (numerator / denominator))
        } else {
            0.
        }
    }

    pub fn apply_friction(particle: &mut Particle) {
        particle.velocity *= FRICTION_MULTIPLIER;
    }
}
