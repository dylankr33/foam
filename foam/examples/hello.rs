#![cfg_attr(target_os = "psp", no_std)]
#![cfg_attr(target_os = "psp", no_main)]

use foam::{
    App, Button, Event, EventHandler, foam_main, lprintln,
    platform::{Box, Error, Vec},
};
use foam_common::FoamCanvas;

#[derive(Default)]
struct Game {
    x: i16,
    y: i16,
}

impl EventHandler for Game {
    fn update(&mut self, context: Vec<Event>) {
        for event in context {
            use Button::*;
            use Event::*;
            match event {
                Pad(button) => match button {
                    Up => self.y -= 1,
                    Down => self.y += 1,
                    Right => self.x += 1,
                    Left => self.x -= 1,
                    _ => (),
                },
                _ => (),
            }
        }
    }
    fn draw(&self, canvas: &mut dyn FoamCanvas) {
        canvas.draw_square(0xff00ff, 30, 30, self.x, self.y)
    }
}

#[foam_main]
fn main() -> Result<(), Box<dyn Error>> {
    let game = Box::new(Game::default());
    let app = App::new(game)?;
    app.run()?;
    Ok(())
}
