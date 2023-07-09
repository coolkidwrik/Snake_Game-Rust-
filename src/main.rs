extern crate rand;             // for random elements
extern crate piston_window;    // for ui

mod draw;
mod snake;
mod game;

use draw::to_coord;
use piston_window::*;
use piston_window::types::Color;

use game::Game;

const BACK_COLOR: Color = [0.05, 0.91, 0.42, 1.0];

//  runs the snake game
fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Rusty Snake",
    [to_coord(width), to_coord(height)]
).exit_on_esc(true).build().unwrap();

let mut game = Game::new(width, height);
while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
        game.key_pressed(key);
    }
    window.draw_2d(&event, |c, g, _| {
        clear(BACK_COLOR, g);
        game.draw(&c, g);
    });

    event.update(|arg| {
        game.update(arg.dt);
    });
}
}
