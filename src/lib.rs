#![cfg_attr(target_os = "psp", no_std)]
#![feature(stmt_expr_attributes)]

pub use foam_proc_macro::*;

#[cfg_retro]
extern crate alloc;

pub mod platform {
    use foam_proc_macro::*;

    #[cfg_retro]
    pub use alloc::boxed::Box;
    #[cfg_retro]
    pub use core::error::Error;
    #[cfg_modern]
    pub use std::boxed::Box;
    #[cfg_modern]
    pub use std::error::Error;
}

use platform::{Box, Error};

pub struct App {
    #[cfg(target_os = "psp")]
    renderer: foam_psp::Renderer,
    #[cfg(not(target_os = "psp"))]
    renderer: i32,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let renderer = {
            #[cfg(target_os = "psp")]
            {
                foam_psp::Renderer::new()?
            }
            #[cfg(not(target_os = "psp"))]
            {
                2
            }
        };
        Ok(App { renderer })
    }
}

pub mod print;
