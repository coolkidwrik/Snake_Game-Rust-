// rectangle is for drawing rectangles
// Context is used to for context of 2d content
// G2d is a graphics buffer, renders the content
use piston_window::{rectangle, Context, G2d};
// types for color [r, g, b, a]
// each input is from 0-1
use piston_window::types::Color;
  
//constant size of a block
// block is used to make up the snake and food
// (and border)
const  BLOCK_SIZE: f64 = 25.0;

// takes in a coordinate, and scales it up by block size(in f64)
// each block will form a coordinate in the game
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

// takes in a color, x and y coordinates, a context, and a mutable graphics buffer
// converts screen coordinates, x and y, to game coordinates
// creates a square(rectangle) at given coordinates with provided color.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y: f64 = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g
    );
}

// takes in a color, x and y coordinates, and a width and a height, and
// a context, and a mutable graphics buffer
//draws a rectangle using these
pub fn draw_rectangle(color: Color, x: i32, y: i32,
    width: i32, height: i32,
    con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, 
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
    con.transform,
g
);

}