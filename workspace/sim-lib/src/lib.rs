mod calc;
mod def;
mod physics;
mod constants;

use femtovg::Canvas;

pub use def::{Particle, ParticleColor, Point, Vector, ForcesConfig, CameraMoveRequest, CameraZoomRequest};
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

    pub fn tick(&mut self) {
        self.update_velocities();
        self.update_positions();
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

    pub fn update_camera_position(&mut self, request: CameraMoveRequest) {
        match request {
            CameraMoveRequest::Down => self.camera_position.y += constants::CAMERA_MOVEMENT_SENSITIVITY / self.scale_factor,
            CameraMoveRequest::Up => self.camera_position.y -= constants::CAMERA_MOVEMENT_SENSITIVITY / self.scale_factor,
            CameraMoveRequest::Right => self.camera_position.x += constants::CAMERA_MOVEMENT_SENSITIVITY / self.scale_factor,
            CameraMoveRequest::Left => self.camera_position.x -= constants::CAMERA_MOVEMENT_SENSITIVITY / self.scale_factor,
        }
    }

    pub fn update_camera_zoom(&mut self, request: CameraZoomRequest) {
        let diff = match request {
            CameraZoomRequest::In => constants::CAMERA_ZOOM_SENSITIVITY,
            CameraZoomRequest::Out => -constants::CAMERA_ZOOM_SENSITIVITY,
        };
        self.scale_factor = calc::bounded(self.scale_factor + diff, constants::MIN_SCALE_FACTOR, constants::MAX_SCALE_FACTOR)
    }

    pub fn set_force_config(&mut self, forces: ForcesConfig) {
        self.forces = forces;
    }

    pub fn get_force_config(&mut self) -> ForcesConfig {
        self.forces
    }

    pub fn accelerate_all(&mut self, amount: f32) {
        let amount = f32::abs(amount);
        self.particles.iter_mut().for_each(|p| p.velocity = p.velocity.with_length(p.velocity.length() + amount));
    }

    fn update_velocities(&mut self) {
        if self.physics_mode == PhysicsMode::Emergence {
            self.particles.iter_mut().for_each(physics::emergence::apply_friction);
        }

        for i in 0..self.particles.len() {
            for j in i + 1..self.particles.len() {
                self.update_velocities_one_way(i, j);
                self.update_velocities_one_way(j, i);
            }
        }
    }

    fn update_velocities_one_way(&mut self, p1_index: usize, p2_index: usize) {
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
