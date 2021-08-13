extern crate rand;
extern crate piston_window;
extern crate find_folder;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;
use find_folder::Search::ParentsThenKids;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake", 
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(false)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    let asset = ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyphs = window.load_font(asset.join("Roboto-Regular.ttf")).unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(d);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
