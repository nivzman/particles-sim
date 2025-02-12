mod app;
mod app_state;
mod constants;

use sim_lib::{ParticleColor, Particle, Point, Vector, Simulation, ForcesConfig, PhysicsMode};
use femtovg::Color;
use app::AppContext;
use app_state::{AppState, CameraZoomRequest};
use winit::{
    event::{ElementState, Event, MouseScrollDelta, TouchPhase, WindowEvent},
    keyboard::KeyCode
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_context = app::init();
    run(app_context)?;
    Ok(())
}

fn run(mut app_context: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    let mut simulation = get_emergence_sim();
    let mut app_state = AppState::new(simulation.get_forces_config());

    let mut time_sum = std::time::Duration::from_millis(0);
    let mut time_count = 0;

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

                    let start = std::time::Instant::now();
                    simulation.tick(Some(&app_state.thread_pool));
                    time_sum += start.elapsed();
                    time_count += 1;
                    if time_count == 50 {
                        println!("Avg tick time: {} milliseconds", time_sum.as_millis() / time_count);
                        time_count = 0;
                        time_sum = std::time::Duration::from_millis(0);
                    }

                    simulation.draw(&mut app_context.canvas, app_state.camera_position, app_state.camera_scale_factor);
                    app_context.surface.present(&mut app_context.canvas).expect("Could not preset canvas to screen");
                },
                WindowEvent::MouseWheel { phase, delta,  .. } => match (phase, delta) {
                    (TouchPhase::Moved, MouseScrollDelta::LineDelta(_, input)) => {
                        if input < 0. {
                            app_state.update_camera_zoom(CameraZoomRequest::Out);
                        } else if input > 0. {
                            app_state.update_camera_zoom(CameraZoomRequest::In);
                        }
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
                } => {
                    if let Ok(req) = key.try_into() {
                        app_state.update_camera_position(req)
                    } else if key == KeyCode::Digit1 {
                        simulation.set_forces_config(ForcesConfig::random(-0.3, 1.0));
                        simulation.accelerate_all(50.0);
                    } else if key == KeyCode::Digit2 {
                        simulation.set_forces_config(app_state.default_forces_config);
                    }
                }
                _ => {}
            },
            _ => {}
        })?;

    Ok(())
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

    for _ in 0..1700 {
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Red));
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Green));
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Blue));
        particles.push(Particle::new(sim_lib::random_world_position(), Vector::new(0., 0.), ParticleColor::Yellow));
    }

    let forces = ForcesConfig::empty()
        .with_force(ParticleColor::Red, ParticleColor::Red, 0.4)
        .with_force(ParticleColor::Blue, ParticleColor::Red, 0.3)
        .with_force(ParticleColor::Yellow, ParticleColor::Red, 0.4)
        .with_force(ParticleColor::Blue, ParticleColor::Blue, 0.3)
        .with_force(ParticleColor::Green, ParticleColor::Green, 0.2)
        .with_force(ParticleColor::Green, ParticleColor::Blue, 0.2)
        .with_force(ParticleColor::Yellow, ParticleColor::Green, 0.4);

    Simulation::new(particles, forces, PhysicsMode::Emergence)
}
