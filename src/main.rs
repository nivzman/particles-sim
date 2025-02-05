mod init;
mod sim;

use std::collections::HashMap;
use sim::{ParticleColor, Particle, Point, Vector, Simulation, ForcesConfiguration, ForceRelation, Physics, WORLD_WIDTH_FLOAT, WORLD_HEIGHT_FLOAT};

use femtovg::Color;
use rand::prelude::ThreadRng;
use init::{AppWindowSurface, AppContext};
use winit::event::{Event, WindowEvent};
use rand::Rng;

fn main() {
    let app_context = init::init();
    run(app_context);
}

// fn rules_function(color1: ParticleColor, color2: ParticleColor) -> Option<f32> {
//     macro_rules! color_match {
//     ($c1:ident, $c2:ident) => {
//         (ParticleColor::$c1, ParticleColor::$c2) | (ParticleColor::$c2, ParticleColor::$c1)
//     };
// }
//     match (color1, color2) {
//         color_match!(Green, Green) => Some(320.),
//         color_match!(Green, Red) => Some(170.),
//         color_match!(Green, Blue) => Some(-140.),
//         color_match!(Red, Red) => Some(100.),
//         color_match!(Red, Green) => Some(340.),
//         color_match!(Blue, Blue) => Some(-150.),
//         _ => None
//     }
// }
//

fn random_position(rng: &mut ThreadRng) -> Point {
    Point::new(rng.random_range(0.0..WORLD_WIDTH_FLOAT), rng.random_range(0.0..WORLD_HEIGHT_FLOAT))
}

fn run<W: AppWindowSurface>(mut app_context: AppContext<W>) {
    let mut particles = Vec::new();

    let mut rng = rand::rng();
    for _ in 0..700 {
        particles.push(Particle::new(random_position(&mut rng), Vector::new(0., 0.), ParticleColor::Red));
        // particles.push(Particle::new(random_position(&mut rng), Vector::new(0., 0.), ParticleColor::Blue));
        particles.push(Particle::new(random_position(&mut rng), Vector::new(0., 0.), ParticleColor::Green));
    }
    // particles.push(Particle::new(Point::new(500., 400.), Vector::new(0., 0.), ParticleColor::Red));
    // particles.push(Particle::new(Point::new(500., 450.), Vector::new(0., 0.), ParticleColor::Red));

    let mut forces: ForcesConfiguration = HashMap::new();
    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Red}, 1.0);
    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Blue}, 0.3);
    forces.insert(ForceRelation { who: ParticleColor::Red, to: ParticleColor::Green}, -1.0);

    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Red}, 0.2);
    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Blue}, 0.2);
    forces.insert(ForceRelation { who: ParticleColor::Blue, to: ParticleColor::Green}, 0.2);

    forces.insert(ForceRelation { who: ParticleColor::Green, to: ParticleColor::Red}, 0.2);
    forces.insert(ForceRelation { who: ParticleColor::Green, to: ParticleColor::Blue}, -0.4);
    forces.insert(ForceRelation { who: ParticleColor::Green, to: ParticleColor::Green}, 0.0);

    let mut simulation = Simulation::new(particles, forces, Physics::Emergence);

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

