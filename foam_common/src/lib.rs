#![no_std]

pub trait FoamRenderer {
    fn clear(&mut self, color: u32);
    fn draw_square(&mut self, color: u32, w: u16, h: u16, x: i16, y: i16);
    fn end_drawing(&mut self);
}
