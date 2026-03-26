#![cfg_attr(target_os = "psp", no_std)]
#![feature(stmt_expr_attributes)]

use foam_common::FoamRenderer;
pub use foam_proc_macro::*;

#[cfg_retro]
extern crate alloc;

pub mod platform {
    use foam_proc_macro::*;

    #[cfg_retro]
    pub use alloc::boxed::Box;
    #[cfg_retro]
    pub use alloc::vec::Vec;
    #[cfg_retro]
    pub use core::error::Error;
    #[cfg_modern]
    pub use std::boxed::Box;
    #[cfg_modern]
    pub use std::error::Error;
}

use foam_psp::Renderer;
use platform::{Box, Error, Vec};

pub struct App {
    canvas: Box<dyn FoamRenderer>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut renderer = {
            #[cfg(target_os = "psp")]
            {
                let mut renderer = foam_psp::Renderer::new()?;
                renderer.init();
                renderer
            }
            #[cfg(not(target_os = "psp"))]
            {
                todo!()
            }
        };

        let canvas = Box::from(renderer);
        Ok(App { canvas })
    }

    pub fn draw(&mut self, color: u32, cb: impl Fn(&mut Box<dyn FoamRenderer>)) {
        self.canvas.clear(color);
        cb(&mut self.canvas);
        self.canvas.end_drawing();
    }
}

pub mod math;
pub mod print;
