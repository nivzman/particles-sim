use std::time::Duration;
use femtovg::Canvas;
use winit::keyboard::KeyCode;
use sim_lib::{World, ForcesConfig, Point, ThreadPool, CalculationMethod, GpuExecutor};
use crate::{
    constants,
    timer::Timer,
};

pub struct App {
    pub world: World,
    pub calculation_method: CalculationMethod,
    pub camera_position: Point,
    pub camera_scale_factor: f32,
    pub default_forces_config: ForcesConfig,
    pub tick_timer: Timer,
    pub draw_timer: Timer,
}

impl App {
    pub fn new(world: World) -> Self {
        let default_forces_config = world.get_forces_config();
        App {
            world,
            calculation_method: CalculationMethod::GPU(GpuExecutor::new()),
            camera_position: Point::new(0., 0.),
            camera_scale_factor: 1.,
            default_forces_config,
            tick_timer: Timer::new(50),
            draw_timer: Timer::new(50),
        }
    }

    pub fn single_world_tick(&mut self) {
        let measurement = self.tick_timer.start();
        self.world.tick(&self.calculation_method);
        measurement.end();
    }

    pub fn draw_world<R: femtovg::Renderer>(&mut self, canvas: &mut Canvas<R>) {
        let measurement = self.draw_timer.start();
        self.world.draw(canvas, self.camera_position, self.camera_scale_factor);
        measurement.end();
    }

    pub fn update_camera_position(&mut self, request: CameraMoveRequest) {
        match request {
            CameraMoveRequest::Down => self.camera_position.y += constants::CAMERA_MOVEMENT_SENSITIVITY / self.camera_scale_factor,
            CameraMoveRequest::Up => self.camera_position.y -= constants::CAMERA_MOVEMENT_SENSITIVITY / self.camera_scale_factor,
            CameraMoveRequest::Right => self.camera_position.x += constants::CAMERA_MOVEMENT_SENSITIVITY / self.camera_scale_factor,
            CameraMoveRequest::Left => self.camera_position.x -= constants::CAMERA_MOVEMENT_SENSITIVITY / self.camera_scale_factor,
        }
    }

    pub fn update_camera_zoom(&mut self, request: CameraZoomRequest) {
        let diff = match request {
            CameraZoomRequest::In => constants::CAMERA_ZOOM_SENSITIVITY,
            CameraZoomRequest::Out => -constants::CAMERA_ZOOM_SENSITIVITY,
        };
        self.camera_scale_factor = sim_lib::bounded_value(self.camera_scale_factor + diff, constants::MIN_CAMERA_SCALE_FACTOR, constants::MAX_CAMERA_SCALE_FACTOR)
    }

    pub fn consume_world_tick_average_time(&mut self) -> Option<Duration> {
        self.tick_timer.consume_average_time()
    }

    pub fn consume_world_draw_average_time(&mut self) -> Option<Duration> {
        self.draw_timer.consume_average_time()
    }
}

pub enum CameraMoveRequest {
    Right,
    Left,
    Up,
    Down,
}

pub enum CameraZoomRequest {
    In,
    Out,
}

impl TryFrom<KeyCode> for CameraMoveRequest {
    type Error = ();

    fn try_from(key: KeyCode) -> Result<Self, Self::Error> {
        Ok(match key {
            KeyCode::KeyS | KeyCode::ArrowDown => CameraMoveRequest::Down,
            KeyCode::KeyW | KeyCode::ArrowUp =>  CameraMoveRequest::Up,
            KeyCode::KeyD | KeyCode::ArrowRight => CameraMoveRequest::Right,
            KeyCode::KeyA | KeyCode::ArrowLeft => CameraMoveRequest::Left,
            _ => return Err(())
        })
    }
}
