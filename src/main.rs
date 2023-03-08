extern crate piston_window;
extern crate rand;

use piston_window::{
    clear, types::Color, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

mod draw;
mod game;
mod snake;

const BLACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [draw::to_coord_u32(width), draw::to_coord_u32(height)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = game::Game::new(width, height);

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&e, |c, g, _| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
