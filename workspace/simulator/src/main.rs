mod init;

use sim_lib::{ParticleColor, Particle, Point, Vector, Simulation, ForcesConfig, PhysicsMode};

use femtovg::Color;
use init::{AppWindowSurface, AppContext};
use winit::event::{ElementState, Event, MouseScrollDelta, TouchPhase, WindowEvent};

fn main() {
    let app_context = init::init();
    run(app_context);
}

fn get_real_sim() -> Simulation {
    let mut particles = Vec::new();

    particles.push(Particle::new(Point::new(500., 500.), Vector::new(0., 0.), ParticleColor::Blue));
    particles.push(Particle::new(Point::new(500., 400.), Vector::new(3., 1.), ParticleColor::Red));

    let forces = ForcesConfig::empty()
        .with_force(ParticleColor::Red, ParticleColor::Blue, 10.)
        .with_force(ParticleColor::Blue, ParticleColor::Red, -0.1);

    Simulation::new(particles, forces, PhysicsMode::Real)
}

fn get_emergence_sim() -> Simulation {
    let mut particles = Vec::new();

    for _ in 0..800 {
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Red));
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Green));
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Blue));
    }

    let forces = ForcesConfig::empty()
        .with_force(ParticleColor::Red, ParticleColor::Red, 0.4)
        .with_force(ParticleColor::Blue, ParticleColor::Red, 0.3)
        .with_force(ParticleColor::Blue, ParticleColor::Blue, 0.3)
        .with_force(ParticleColor::Green, ParticleColor::Green, 0.2)
        .with_force(ParticleColor::Green, ParticleColor::Blue, 0.2);

    Simulation::new(particles, forces, PhysicsMode::Emergence)
}

fn run<W: AppWindowSurface>(mut app_context: AppContext<W>) {
    //let mut simulation = get_real_sim();
    let mut simulation = get_emergence_sim();

    let ticker_thread_window = app_context.window.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
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
                    simulation.tick();
                    simulation.draw(&mut app_context.canvas);
                    app_context.surface.present(&mut app_context.canvas);
                },
                WindowEvent::MouseWheel { phase, delta,  .. } => match (phase, delta) {
                    (TouchPhase::Moved, MouseScrollDelta::LineDelta(_, input)) => {
                        simulation.update_scale_factor(input);
                    }
                    _ => {}
                },
                WindowEvent::KeyboardInput {
                    event: winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => simulation.update_camera_position(key),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
