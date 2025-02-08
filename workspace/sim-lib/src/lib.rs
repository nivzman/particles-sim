mod calc;
mod def;
mod physics;
mod constants;

use femtovg::Canvas;
use winit::keyboard::KeyCode;

pub use def::{Particle, ParticleColor, Point, Vector, ForcesConfig};
pub use physics::PhysicsMode;
pub use calc::{random_world_position};

    pub struct Simulation {
    particles: Vec<Particle>,
    forces: ForcesConfig,
    physics_mode: PhysicsMode,
    camera_position: Point,
    scale_factor: f32,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>, forces: ForcesConfig, physics: PhysicsMode) -> Self {
        Simulation {
            particles,
            forces,
            physics_mode: physics,
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
            path.circle((particle.position.x - self.camera_position.x) * self.scale_factor, (particle.position.y - self.camera_position.y) * self.scale_factor, constants::BASE_PARTICLE_RADIUS * self.scale_factor);
            canvas.fill_path(&path, &femtovg::Paint::color(particle.color.into()));
        }
    }

    pub fn update_camera_position(&mut self, pressed_key: KeyCode) {
        match pressed_key {
            KeyCode::KeyS | KeyCode::ArrowDown => self.camera_position.y += constants::MOVEMENT_SENSITIVITY / self.scale_factor,
            KeyCode::KeyW | KeyCode::ArrowUp => self.camera_position.y -= constants::MOVEMENT_SENSITIVITY / self.scale_factor,
            KeyCode::KeyD | KeyCode::ArrowRight => self.camera_position.x += constants::MOVEMENT_SENSITIVITY / self.scale_factor,
            KeyCode::KeyA | KeyCode::ArrowLeft => self.camera_position.x -= constants::MOVEMENT_SENSITIVITY / self.scale_factor,
            _ => {},
        }
    }

    pub fn update_scale_factor(&mut self, input: f32) {
        self.scale_factor = calc::bounded(self.scale_factor + input * constants::SCALING_SENSITIVITY, constants::MIN_SCALE_FACTOR, constants::MAX_SCALE_FACTOR)
    }

    pub fn tick(&mut self) {
        self.update_velocities();
        self.update_positions();
    }

    fn update_velocities(&mut self) {
        if self.physics_mode == PhysicsMode::Emergence {
            self.particles.iter_mut().for_each(physics::emergence::apply_friction);
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

        let distance = p1.position.distance_to(p2.position) / constants::WORLD_UNIT_SIZE;
        let configured_force = self.forces.get(p1.color, p2.color);

        let force = match self.physics_mode {
            PhysicsMode::Emergence => physics::emergence::calculate_force(configured_force, distance),
            PhysicsMode::Real => physics::real::calculate_force(configured_force, distance),
        };

        if force == 0. {
            return;
        }

        let force = force * constants::FORCE_SCALAR;

        let p1_to_p2_vec = p2.position - p1.position;
        let p1_acceleration = Vector::from_angle_and_length(p1_to_p2_vec.angle_from_x_axis(), force);
        self.particles[p1_index].velocity += p1_acceleration;
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
}
