extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Island Zero", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            // stuff goes here
        });
    }
}