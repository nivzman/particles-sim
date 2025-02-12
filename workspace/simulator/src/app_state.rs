use winit::keyboard::KeyCode;
use sim_lib::{ForcesConfig, Point, ThreadPool};
use crate::constants;

pub struct AppState {
    pub thread_pool: ThreadPool,
    pub camera_position: Point,
    pub camera_scale_factor: f32,
    pub default_forces_config: ForcesConfig
}

impl AppState {
    pub fn new(default_forces_config: ForcesConfig) -> Self {
        AppState {
            thread_pool: ThreadPool::new(num_cpus::get()),
            camera_position: Point::new(0., 0.),
            camera_scale_factor: 1.,
            default_forces_config
        }
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
