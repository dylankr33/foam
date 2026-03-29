use std::{error::Error, ffi::CString};

use foam_common::FoamRenderer;
use glutin::{
    config::Config,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::{GlContext, GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, WindowSurface},
};
use glutin_winit::GlWindow;
use winit::raw_window_handle::HasWindowHandle;

const VERTICES: [Vertex; 3] = [
    Vertex { x: -1, y: -1, z: 0 },
    Vertex { x: 1, y: -1, z: 0 },
    Vertex { x: 0, y: 1, z: 0 },
];

#[repr(C)]
struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}

pub struct GlApp {
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
}

fn hex_to_float(color: u32) -> (f32, f32, f32) {
    let r = (color & 0xff) as f32 / 255.0;
    let g = ((color >> 8) & 0xff) as f32 / 255.0;
    let b = ((color >> 16) & 0xff) as f32 / 255.0;
    (r, g, b)
}

impl GlApp {
    pub fn new(window: &winit::window::Window, gl_config: Config) -> Result<Self, Box<dyn Error>> {
        let raw_window_handle = window.window_handle()?.as_raw();
        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let gl_display = gl_config.display();
        let gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)?
                .treat_as_possibly_current()
        };
        gl::load_with(|f| {
            gl_context
                .display()
                .get_proc_address(CString::new(f).unwrap().as_c_str())
        });
        let attrs = window.build_surface_attributes(Default::default())?;
        let gl_surface = unsafe { gl_display.create_window_surface(&gl_config, &attrs)? };
        unsafe {
            let _ = gl_context.make_current(&gl_surface);
            gl::Viewport(0, 0, 800, 600);
        }
        Ok(Self {
            gl_context,
            gl_surface,
        })
    }
}

impl FoamRenderer for GlApp {
    fn clear(&mut self, color: u32) {
        unsafe {
            let (r, g, b) = hex_to_float(color);
            gl::ClearColor(r, r, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            let _ = self.gl_surface.swap_buffers(&self.gl_context);
        }
    }
    fn draw_square(&mut self, color: u32, w: u16, h: u16, x: i16, y: i16) {}
    fn end_drawing(&mut self) {}
    /// This is all handled by winit
    fn poll_event(&mut self) -> Vec<foam_common::Event> {
        unimplemented!()
    }
}
