mod calc;
mod def;

use femtovg::Canvas;

use def::{WorldEdge, PARTICLE_RADIUS, WORLD_WIDTH_BOUND, WORLD_HEIGHT_BOUND};

pub use def::{ParticleColor, Point, Vector, WORLD_WIDTH, WORLD_HEIGHT, WORLD_WIDTH_FLOAT, WORLD_HEIGHT_FLOAT, ForceRelation, Forces};


mod physics_consts
{
    pub const MAX_APPLIED_FORCE: f32 = 0.1;
    pub mod real
    {
    }

    pub mod emergence
    {
        pub const GLOBAL_FORCE: f32 = -0.;
        pub const DRAG_FORCE: f32 = 0.005;
    }
}


pub struct Particle {
    position: Point,
    velocity: Vector,
    color: ParticleColor,
}
pub struct Simulation {
    particles: Vec<Particle>,
    forces: Forces,
    physics: Physics,
}

#[derive(Eq, PartialEq)]
pub enum Physics {
    Real,
    Emergence,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>, forces: Forces, physics: Physics) -> Self {
        Simulation {
            particles,
            forces,
            physics,
        }
    }

    pub fn draw<R: femtovg::Renderer>(&self, canvas: &mut Canvas<R>) {
        for particle in self.particles.iter() {
            let mut path = femtovg::Path::new();
            path.circle(particle.position.x, particle.position.y, PARTICLE_RADIUS);
            canvas.fill_path(&path, &femtovg::Paint::color(particle.color.into()));
        }
    }

    pub fn update_single_tick(&mut self) {
        self.update_velocities();
        self.update_positions();
    }

    fn update_velocities(&mut self) {
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

        let distance = p1.position.distance_to(p2.position);
        if distance == 0. {
            return;
        }

        let mut force = self.forces.get(&ForceRelation { who: p1.color, to: p2.color}).map(|f| *f).unwrap_or(0.);

        if self.physics == Physics::Emergence {
            force += physics_consts::emergence::GLOBAL_FORCE;
        }

        let force = force / distance.powi(2);
        let force = calc::float_min(force, physics_consts::MAX_APPLIED_FORCE);
        let force = calc::float_max(force, -physics_consts::MAX_APPLIED_FORCE);

        let p1_to_p2_vec = p2.position - p1.position;
        let p1_acceleration = Vector::from_angle_and_length(p1_to_p2_vec.angle_from_x_axis(), force);

        let p1 = &mut self.particles[p1_index];
        p1.velocity += p1_acceleration;

        if self.physics == Physics::Real {
            return;
        }

        p1.velocity = p1.velocity.with_length(calc::float_max(0., p1.velocity.length() - physics_consts::emergence::DRAG_FORCE));
    }

    fn update_positions(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.position += particle.velocity;
            if let Some(edge) = calc::check_out_of_bounds(&particle.position) {
                match edge {
                    WorldEdge::Left | WorldEdge::Right => {
                        particle.velocity.x = -particle.velocity.x;
                        if edge == WorldEdge::Right {
                            particle.position.x = WORLD_WIDTH_BOUND
                        } else {
                            particle.position.x = PARTICLE_RADIUS
                        }
                    },
                    WorldEdge::Top | WorldEdge::Bottom => {
                        particle.velocity.y = -particle.velocity.y;
                        if edge == WorldEdge::Bottom {
                            particle.position.y = WORLD_HEIGHT_BOUND
                        } else {
                            particle.position.y = PARTICLE_RADIUS
                        }
                    }
                }
            }
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
