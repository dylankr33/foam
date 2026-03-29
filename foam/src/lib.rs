#![cfg_attr(target_os = "psp", no_std)]

pub use foam_common::{Button, Event, EventHandler};
use foam_common::{FoamBackend, FoamCanvas};
pub use foam_proc_macro::*;

#[cfg_retro]
extern crate alloc;

/// `std` and `core` types that are used in foam
pub mod platform {
    #[cfg_retro]
    pub use alloc::{boxed::Box, vec::Vec};
    #[cfg_retro]
    pub use core::error::Error;
    use foam_proc_macro::*;
    #[cfg_modern]
    pub use std::{boxed::Box, cell::Cell, error::Error, vec::Vec};
}

#[cfg(target_os = "psp")]
use foam_psp::PspBackend;

use platform::{Box, Error, Vec};
#[cfg(any(windows, unix))]
use winit::{event_loop::EventLoop, window::Window};

pub struct App {
    game: Box<dyn EventHandler>,
    renderer: Box<dyn FoamBackend>,
    #[cfg(any(windows, unix))]
    event_loop: EventLoop<()>,
    #[cfg(any(windows, unix))]
    window: Window,
}

impl App {
    pub fn new(game: Box<dyn EventHandler>) -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "gl")]
        let (event_loop, window, gl_config) = {
            let event_loop = EventLoop::new()?;
            use glutin::config::ConfigTemplateBuilder;
            use glutin_winit::DisplayBuilder;
            use winit::{dpi::LogicalSize, window::WindowAttributes};
            let window_attributes = WindowAttributes::default()
                .with_title("Foam App")
                .with_min_inner_size(LogicalSize::new(800, 600));
            let (window, gl_config) = DisplayBuilder::new()
                .with_window_attributes(Some(window_attributes))
                .build(&event_loop, ConfigTemplateBuilder::new(), |configs| {
                    configs
                        .peekable()
                        .next()
                        .expect("Could not find a valid config!")
                })?;
            let Some(window) = window else {
                panic!("Could not make window!")
            };
            (event_loop, window, gl_config)
        };
        let renderer = {
            #[cfg(target_os = "psp")]
            {
                let mut renderer = foam_psp::PspBackend::new()?;
                renderer
            }
            #[cfg(feature = "gl")]
            {
                let renderer = foam_gl::GlApp::new(&window, gl_config)?;
                renderer
            }
            #[cfg(not(any(target_os = "psp", windows, unix)))]
            {
                todo!()
            }
        };
        Ok(App {
            game,
            renderer: Box::new(renderer),
            #[cfg(any(windows, unix))]
            event_loop,
            #[cfg(any(windows, unix))]
            window,
        })
    }

    #[allow(unused_mut)]
    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        #[cfg(target_os = "psp")]
        {
            loop {
                self.game.update(self.renderer.poll_event());
                self.renderer.draw(&|canvas| self.game.draw(canvas));
            }
        }
        #[cfg(any(windows, unix))]
        {
            use winit::event::{Event as WEvent, WindowEvent};
            let _ = self.event_loop.run(|event, elwt| match event {
                WEvent::AboutToWait => self.window.request_redraw(),
                WEvent::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        if !elwt.exiting() {
                            self.canvas.clear(0xffffff);
                            self.game.draw(&mut self.canvas);
                            self.canvas.end_drawing();
                        }
                    }
                    _ => (),
                },
                _ => (),
            });
        }
        Ok(())
    }
}

pub mod math;
pub mod print;
