mod calc;
mod def;

use femtovg::Canvas;

use def::{WorldEdge, PARTICLE_RADIUS, WORLD_WIDTH_BOUND, WORLD_HEIGHT_BOUND};

pub use def::{ParticleColor, Point, Vector, WORLD_WIDTH, WORLD_HEIGHT, ForceRelation, Forces};


const MAX_APPLIED_FORCE: f32 = 0.1;
const MAX_VELOCITY: f32 = 5.;

pub struct Particle {
    position: Point,
    velocity: Vector,
    color: ParticleColor,
}
pub struct Simulation {
    particles: Vec<Particle>,
    forces: Forces,
}

impl Simulation {
    pub fn new(forces: Forces, particles: Vec<Particle>) -> Self {
        Simulation {
            particles,
            forces
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

        let Some(force) = self.forces.get(&ForceRelation { who: p1.color, to: p2.color}) else {
            return;
        };

        let force = force / distance.powi(2);
        let force = calc::float_min(force, MAX_APPLIED_FORCE);
        let force = calc::float_max(force, -MAX_APPLIED_FORCE);

        let p1_to_p2_vec = p2.position - p1.position;
        let p1_acceleration = Vector::from_angle_and_length(p1_to_p2_vec.angle_from_x_axis(), force);
        self.particles[p1_index].velocity += p1_acceleration;
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
