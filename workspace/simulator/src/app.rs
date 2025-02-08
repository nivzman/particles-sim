use std::{
    sync::Arc,
    num::NonZeroU32,
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
    dpi::PhysicalSize,
};
use femtovg::{
    Canvas,
    renderer::OpenGl,
};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

pub struct OpenGlWindowSurface {
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl OpenGlWindowSurface {
    pub fn present(&self, canvas: &mut Canvas<OpenGl>) -> Result<(), Box<dyn std::error::Error>> {
        canvas.flush();
        self.surface.swap_buffers(&self.context)?;
        Ok(())
    }
}

pub struct AppContext {
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
    pub canvas: Canvas<OpenGl>,
    pub surface: OpenGlWindowSurface,
}

pub fn init() -> AppContext {
    let event_loop = EventLoop::new().expect("Could not create event loop");

    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_title("Simulator");

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |mut configs| configs.next().expect("No display config"))
        .expect("Window build failed");

    let window = window.expect("No window could be retrieved");

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));

    let mut not_current_gl_context = Some(unsafe { gl_display.create_context(&gl_config, &context_attributes).expect("OpenGl create context failed") });

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(WINDOW_WIDTH).expect("Zero value provided"),
        NonZeroU32::new(WINDOW_HEIGHT).expect("Zero value provided"),
    );


    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).expect("OpenGl create window surface failed")  };

    let context = not_current_gl_context.take()
        .expect("No OpenGl context")
        .make_current(&surface)
        .expect("OpenGl make current context failed");

    let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast()) }
        .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer)
        .expect("Cannot create canvas");

    canvas.set_size(WINDOW_WIDTH, WINDOW_HEIGHT, window.scale_factor() as f32);

    AppContext {
        event_loop,
        window: Arc::new(window),
        canvas,
        surface: OpenGlWindowSurface { context, surface }
    }
}
