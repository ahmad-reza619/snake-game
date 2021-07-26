use piston_window::{Glyphs, Transformed, rectangle, Context, G2d};
use piston_window::text::*;
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;
const TEXT_COLOR: Color = [0.0, 1.0, 0.0, 1.0];

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    );
}

pub fn draw_text(text: &str, glyph: &mut Glyphs, c: &Context, g: &mut G2d) {
    let transform = c.transform.trans(10.0, 32.0);

    println!("{}", text);

    Text::new_color(TEXT_COLOR, 32)
        .draw(text, glyph, &c.draw_state, transform, g)
        .unwrap();
}