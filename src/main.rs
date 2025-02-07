mod init;
mod sim;

use std::collections::HashMap;
use sim::{ParticleColor, Particle, Point, Vector, Simulation, ForcesConfiguration, ForceRelation, Physics};

use femtovg::Color;
use init::{AppWindowSurface, AppContext};
use winit::event::{Event, WindowEvent};
use rand::Rng;

fn main() {
    let app_context = init::init();
    run(app_context);
}

fn get_real_sim() -> Simulation {
    let mut particles = Vec::new();

    particles.push(Particle::new(Point::new(500., 500.), Vector::new(0., 0.), ParticleColor::Blue));
    particles.push(Particle::new(Point::new(500., 400.), Vector::new(3., 1.), ParticleColor::Red));

    let mut forces: ForcesConfiguration = HashMap::new();
    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Blue}, 10.);
    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Red}, -0.1);

    Simulation::new(particles, forces, Physics::Real)
}

fn get_emergence_sim() -> Simulation {
    let mut particles = Vec::new();

    for _ in 0..400 {
        particles.push(Particle::new(sim::random_position(), Vector::new(0., 0.), ParticleColor::Red));
        particles.push(Particle::new(sim::random_position(), Vector::new(0., 0.), ParticleColor::Green));
        particles.push(Particle::new(sim::random_position(), Vector::new(0., 0.), ParticleColor::Blue));
    }

    let mut forces: ForcesConfiguration = HashMap::new();

    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Red}, 1.0);
    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Green}, 0.0);
    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Blue}, 0.0);

    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Red}, 0.6);
    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Green}, 0.0);
    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Blue}, 1.0);

    forces.insert(ForceRelation { who: ParticleColor::Green, to: ParticleColor::Red}, 0.0);
    forces.insert(ForceRelation { who: ParticleColor::Green, to: ParticleColor::Green}, 1.0);
    forces.insert(ForceRelation { who: ParticleColor::Green, to: ParticleColor::Blue}, 0.6);

    Simulation::new(particles, forces, Physics::Emergence)
}

fn run<W: AppWindowSurface>(mut app_context: AppContext<W>) {
    //let mut simulation = get_real_sim();
    let mut simulation = get_emergence_sim();

    let ticker_thread_window = app_context.window.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));
            ticker_thread_window.request_redraw();
        }
    });

    app_context.event_loop
        .run(move |event, event_target_window| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => event_target_window.exit(),
                WindowEvent::RedrawRequested { .. } => {
                    let size = app_context.window.inner_size();
                    app_context.canvas.set_size(size.width, size.height, app_context.window.scale_factor() as f32);
                    app_context.canvas.clear_rect(0, 0, size.width, size.height, Color::black());
                    simulation.update_single_tick();
                    simulation.draw(&mut app_context.canvas);
                    app_context.surface.present(&mut app_context.canvas);
                }
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}

