#![no_std]

extern crate alloc;

use alloc::vec::Vec;

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

pub trait FoamRenderer {
    fn clear(&mut self, color: u32);
    fn draw_square(&mut self, color: u32, w: u16, h: u16, x: i16, y: i16);
    fn end_drawing(&mut self);
    fn poll_event(&mut self) -> Vec<Event>;
}
pub fn rgb_to_abgr(color: u32) -> u32 {
    let b = color & 0xFF;
    let g = (color >> 8) & 0xFF;
    let r = (color >> 16) & 0xFF;
    return (0xFF << 24) | (b << 16) | (g << 8) | r;
}
