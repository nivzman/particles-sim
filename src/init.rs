use std::num::NonZeroU32;
use std::sync::Arc;

use femtovg::renderer::OpenGl;
use femtovg::{Canvas};
use glutin::surface::Surface;
use glutin::{context::PossiblyCurrentContext};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{dpi::PhysicalSize, window::Window};

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

use crate::sim::{WORLD_HEIGHT, WORLD_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};


pub trait AppWindowSurface {
    type Renderer: femtovg::Renderer + 'static;
    fn present(&self, canvas: &mut Canvas<Self::Renderer>);
}

pub struct OpenGlWindowSurface {
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl AppWindowSurface for OpenGlWindowSurface {
    type Renderer = OpenGl;

    fn present(&self, canvas: &mut Canvas<Self::Renderer>) {
        canvas.flush();
        self.surface.swap_buffers(&self.context).unwrap();
    }
}

pub struct AppContext<W: AppWindowSurface> {
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
    pub canvas: Canvas<W::Renderer>,
    pub surface: W,
}

pub fn init() -> AppContext<OpenGlWindowSurface> {
    let event_loop = EventLoop::new().unwrap();

    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_title("Femtovg");

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));

    let mut not_current_gl_context =
        Some(unsafe { gl_display.create_context(&gl_config, &context_attributes).unwrap() });

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(WINDOW_WIDTH).unwrap(),
        NonZeroU32::new(WINDOW_HEIGHT).unwrap(),
    );

    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

    let context = not_current_gl_context.take().unwrap().make_current(&surface).unwrap();

    let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast()) }
        .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(WORLD_WIDTH, WORLD_HEIGHT, window.scale_factor() as f32);

    AppContext {
        event_loop,
        window: Arc::new(window),
        canvas,
        surface: OpenGlWindowSurface { context, surface }
    }
}
