mod calc;
mod def;

use femtovg::Canvas;
use def::WorldEdge;

pub use def::{Particle, ParticleColor, Point, Vector, WORLD_WIDTH, WORLD_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH, ForcesConfig};
pub use calc::{random_position};

use crate::sim::def::{WORLD_HEIGHT_FLOAT, WORLD_WIDTH_FLOAT};

mod physics_consts
{
    pub const WORLD_SINGLE_UNIT_SIZE_IN_PIXELS: f32 = 100.0;
    pub const FORCE_SCALAR: f32 = 0.25;

    pub mod real
    {
        pub const MAX_APPLIED_FORCE: f32 = 0.1;
    }

    pub mod emergence
    {
        pub const FRICTION_MULTIPLIER: f32 = 0.6;
        pub const COMMON_REPEL_FORCE_RADIUS: f32 = 0.3;
    }
}

mod hud_consts
{
    pub const BASE_PARTICLE_RADIUS: f32 = 3.;
    pub const MAX_SCALE_FACTOR: f32 = 3.0;
    pub const MIN_SCALE_FACTOR: f32 = 0.1;
    pub const MOVEMENT_SENSITIVITY: f32 = 20.0;
    pub const SCALING_SENSITIVITY: f32 = 0.05;
}

pub struct Simulation {
    particles: Vec<Particle>,
    forces: ForcesConfig,
    physics: Physics,
    camera_position: Point,
    scale_factor: f32,
}

#[derive(Eq, PartialEq)]
pub enum Physics {
    Real,
    Emergence,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>, forces: ForcesConfig, physics: Physics) -> Self {
        Simulation {
            particles,
            forces,
            physics,
            camera_position: Point::new(0., 0.),
            scale_factor: 1.,
        }
    }

    pub fn draw<R: femtovg::Renderer>(&self, canvas: &mut Canvas<R>) {
        let min_x = self.camera_position.x;
        let max_x = self.camera_position.x + (canvas.width() as f32 / self.scale_factor);
        let min_y = self.camera_position.y;
        let max_y = self.camera_position.y + (canvas.height() as f32 / self.scale_factor);

        for particle in self.particles.iter() {
            if particle.position.x < min_x || particle.position.x > max_x || particle.position.y < min_y || particle.position.y > max_y {
                continue;
            }

            let mut path = femtovg::Path::new();
            path.circle((particle.position.x - self.camera_position.x) * self.scale_factor, (particle.position.y - self.camera_position.y) * self.scale_factor, hud_consts::BASE_PARTICLE_RADIUS * self.scale_factor);
            canvas.fill_path(&path, &femtovg::Paint::color(particle.color.into()));
        }
    }

    pub fn update_camera_position(&mut self, pressed_key: winit::keyboard::KeyCode) {
        match pressed_key {
            winit::keyboard::KeyCode::KeyS => self.camera_position.y += hud_consts::MOVEMENT_SENSITIVITY / self.scale_factor,
            winit::keyboard::KeyCode::KeyW => self.camera_position.y -= hud_consts::MOVEMENT_SENSITIVITY / self.scale_factor,
            winit::keyboard::KeyCode::KeyD => self.camera_position.x += hud_consts::MOVEMENT_SENSITIVITY / self.scale_factor,
            winit::keyboard::KeyCode::KeyA => self.camera_position.x -= hud_consts::MOVEMENT_SENSITIVITY / self.scale_factor,
            _ => {},
        }
    }

    pub fn update_scale_factor(&mut self, input: f32) {
        self.scale_factor = calc::float_max(calc::float_min(self.scale_factor + input * hud_consts::SCALING_SENSITIVITY, hud_consts::MAX_SCALE_FACTOR), hud_consts::MIN_SCALE_FACTOR);
    }

    pub fn update_single_tick(&mut self) {
        self.update_velocities();
        self.update_positions();
    }

    fn update_velocities(&mut self) {
        if self.physics == Physics::Emergence {
            for particle in self.particles.iter_mut() {
                particle.velocity *= physics_consts::emergence::FRICTION_MULTIPLIER;
            }
        }

        for i in 0..self.particles.len() {
            for j in i + 1..self.particles.len() {
                self.update_velocities_for_pair(i, j);
                self.update_velocities_for_pair(j, i);
            }
        }
    }

    fn update_velocities_for_pair(&mut self, p1_index: usize, p2_index: usize) {
        let p1 = &self.particles[p1_index];
        let p2 = &self.particles[p2_index];

        let distance = p1.position.distance_to(p2.position) / physics_consts::WORLD_SINGLE_UNIT_SIZE_IN_PIXELS;
        let configured_force = self.forces.get(p1.color, p2.color);

        let force = match self.physics {
            Physics::Emergence => Self::calculate_force_emergence(configured_force, distance),
            Physics::Real => Self::calculate_force_real(configured_force, distance),
        };

        if force == 0. {
            return;
        }

        let force = force * physics_consts::FORCE_SCALAR;

        let p1_to_p2_vec = p2.position - p1.position;
        let p1_acceleration = Vector::from_angle_and_length(p1_to_p2_vec.angle_from_x_axis(), force);
        self.particles[p1_index].velocity += p1_acceleration;
    }

    fn calculate_force_real(configured_attraction_force: f32, distance: f32) -> f32 {
        if distance == 0. {
            return 0.;
        }

        let force = configured_attraction_force / distance.powi(2);
        let force = calc::float_min(force, physics_consts::real::MAX_APPLIED_FORCE);
        calc::float_max(force, -physics_consts::real::MAX_APPLIED_FORCE)
    }

    fn calculate_force_emergence(configured_attraction_force: f32, distance: f32) -> f32 {
        if distance <= physics_consts::emergence::COMMON_REPEL_FORCE_RADIUS {
            (distance / physics_consts::emergence::COMMON_REPEL_FORCE_RADIUS) - 1.0
        } else if distance < 1. {
            let numerator = f32::abs((2. * distance) - 1. - physics_consts::emergence::COMMON_REPEL_FORCE_RADIUS);
            let denominator = 1. - physics_consts::emergence::COMMON_REPEL_FORCE_RADIUS;
            let l = 1. - (numerator / denominator);
            configured_attraction_force * l
        } else {
            0.
        }
    }

    fn update_positions(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.position += particle.velocity;
            match self.physics {
                Physics::Real => Self::handle_out_of_bounds_real(particle),
                Physics::Emergence  => Self::handle_out_of_bounds_emergence(particle),
            }
        }
    }

    fn handle_out_of_bounds_emergence(particle: &mut Particle) {
        if !calc::is_out_of_bounds(&particle.position) {
            return;
        }

        particle.position = random_position();
    }

    fn handle_out_of_bounds_real(particle: &mut Particle) {
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
