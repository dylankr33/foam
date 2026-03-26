#![cfg_attr(target_os = "psp", no_std)]
#![cfg_attr(target_os = "psp", no_main)]

use foam::{App, cfg_retro, foam_main, lprintln, math};

#[foam_main]
fn main() {
    let mut app = App::new().expect("Couldn't create app!");
    let mut i = 0;
    loop {
        i += 1;
        let sin = unsafe { math::sin32(0.1 * (i) as f32) * 15.0 };
        app.draw(0xffffff, |canvas| {
            canvas.draw_square(0xaaaaaa, 100, 100, 40, sin as i16);
            canvas.draw_square(0x223344, 100, 100, 200, 67);
        });
    }
}
