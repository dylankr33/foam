#![no_std]

extern crate alloc;

pub mod shapes;

use alloc::{boxed::Box, vec::Vec};
use core::any::Any;

use crate::prelude::Event;
pub mod prelude {
    pub enum Button {
        X,
        Y,
        A,
        B,
        Up,
        Down,
        Right,
        Left,
    }

    pub enum Event {
        Quit,
        None,
        Pad(Button),
    }

    #[repr(C)]
    pub struct Vertex {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub u: f32,
        pub v: f32,
    }
}

pub trait EventHandler {
    fn update(&mut self, context: Vec<Event>);
    fn draw(&self, canvas: &mut dyn FoamCanvas);
}

pub trait FoamCanvas {
    fn draw_cube(&self, color: u32, position: (f32, f32, f32)) {}
}

pub trait FoamBackend {
    fn poll_event(&mut self) -> Vec<Event>;
    fn draw(&mut self, cb: &dyn Fn(&mut dyn FoamCanvas));
}
pub fn rgb_to_abgr(color: u32) -> u32 {
    let b = color & 0xFF;
    let g = (color >> 8) & 0xFF;
    let r = (color >> 16) & 0xFF;
    return (0xFF << 24) | (b << 16) | (g << 8) | r;
}
