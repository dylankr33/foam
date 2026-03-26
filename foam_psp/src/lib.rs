#![no_std]

use core::error::Error;

use alloc::boxed::Box;

extern crate alloc;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }
}
