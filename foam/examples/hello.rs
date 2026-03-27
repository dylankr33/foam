#![cfg_attr(target_os = "psp", no_std)]
#![cfg_attr(target_os = "psp", no_main)]

use foam::{
    App, Button, Event, EventHandler, foam_main,
    platform::{Box, Error, Vec},
};

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
    fn draw(&mut self, context: &mut Box<dyn foam_common::FoamRenderer>) {
        context.draw_square(0xaaaaaa, 100, 100, self.x, self.y);
        context.draw_square(0x223344, 100, 100, 200, 67);
    }
}

#[foam_main]
fn main() -> Result<(), Box<dyn Error>> {
    let game = Box::new(Game::default());
    let app = App::new(game)?;
    app.run()?;
    Ok(())
}
