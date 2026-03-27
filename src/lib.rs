#![feature(stmt_expr_attributes)]
#![cfg_attr(target_os = "psp", no_std)]

use foam_common::FoamRenderer;
pub use foam_common::{Button, Event};
pub use foam_proc_macro::*;

#[cfg_retro]
extern crate alloc;

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
use foam_psp::Renderer;

use platform::{Box, Error, Vec};
#[cfg(any(windows, unix))]
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub trait EventHandler {
    fn update(&mut self, context: Vec<Event>);
    fn draw(&mut self, context: &mut Box<dyn FoamRenderer>);
}

pub struct App {
    game: Box<dyn EventHandler>,
    canvas: Box<dyn FoamRenderer>,
    #[cfg(any(windows, unix))]
    event_loop: EventLoop<()>,
    #[cfg(any(windows, unix))]
    window: Window,
}

impl App {
    pub fn new(game: Box<dyn EventHandler>) -> Result<Self, Box<dyn Error>> {
        let renderer = {
            #[cfg(target_os = "psp")]
            {
                let mut renderer = foam_psp::Renderer::new()?;
                renderer.init();
                renderer
            }
            #[cfg(any(windows, unix))]
            {
                pretty_env_logger::init();

                let renderer = foam_vk::VkApp::new()?;
                renderer
            }
            #[cfg(not(any(target_os = "psp", windows, unix)))]
            {
                todo!()
            }
        };
        #[cfg(any(windows, unix))]
        let (event_loop, window) = {
            use winit::dpi::LogicalSize;

            let event_loop = EventLoop::new()?;
            let window = WindowBuilder::new()
                .with_title("Foam App")
                .with_inner_size(LogicalSize::new(800, 800))
                .build(&event_loop)?;
            (event_loop, window)
        };
        Ok(App {
            game,
            canvas: Box::from(renderer),
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
                self.game.update(self.canvas.poll_event());
                self.canvas.clear(0xffffff);
                self.game.draw(&mut self.canvas);
                self.canvas.end_drawing();
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
                        if elwt.exiting() {
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
