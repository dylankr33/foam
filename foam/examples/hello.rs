#![cfg_attr(target_os = "psp", no_std)]
#![cfg_attr(target_os = "psp", no_main)]

use foam::{
    App, Button, Event, EventHandler, FoamCanvas, foam_main, lprintln,
    platform::{Box, Error, Vec},
};

#[derive(Default)]
struct Game {
    x: f32,
    y: f32,
}

impl EventHandler for Game {
    fn update(&mut self, context: Vec<Event>) {
        for event in context {
            use Event::*;
            match event {
                Pad(button) => match button {
                    Button::Up => self.y -= 0.2,
                    Button::Down => self.y += 0.2,
                    Button::Right => self.x += 0.2,
                    Button::Left => self.x -= 0.2,
                    _ => (),
                },
                _ => (),
            }
        }
    }
    fn draw(&self, canvas: &mut dyn FoamCanvas) {
        canvas.draw_cube(0xff00000, (self.x, -2.0, self.y))
    }
}

#[foam_main]
fn main() -> Result<(), Box<dyn Error>> {
    let game = Box::new(Game::default());
    let app = App::new(game)?;
    app.run()?;
    Ok(())
}
