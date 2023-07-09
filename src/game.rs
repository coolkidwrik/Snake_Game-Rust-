// import everything from piston_window
use piston_window::*;
use piston_window::types::Color;

// allows to create a thread local, random number
use rand::{thread_rng, Rng};

// import snake and draw from snake.rs and draw.rs
use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

//constants
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.6, 0.4, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.3];

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0; // after game end, time to start again

// fields for game object
pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width :i32,
    height: i32,

    game_over: bool,
    waiting_time: f64
}

// methods for game object
impl Game {

    // constructor for the game, that takes in a width and a height
    pub fn new(width: i32, height: i32) -> Game {
        Game { snake: Snake::new(2, 2),
             food_exists: true,
             food_x: 6, 
             food_y: 4, 
             width, height, 
             game_over: false, 
             waiting_time: 0.0 }
    }

    // checks to see if user has pressed a key, and reacts as specified
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        // takes in arrow key inputs and returns the corresponding new direction
        // also handles aswd inputs
        let dir = match key{
            Key::Up => Some(Direction::Up),
            Key::W => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::S => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::A => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::D => Some(Direction::Right),
            _ => None
        };

        // if the new direction is the opposite to old direction, do nothing
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        // else, update snake direction to new direction
        self.update_snake(dir);
    }

    // draw the snake, food, and borders of the game
    // incase of game over, incase the game window in a red box
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // update all elements of the game
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // check to see if the snake has eaten, then update snake
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    //check to see of snake is alive
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self. width - 1 && next_y < self.height -1
    }

    // add food at a random spot in the game window
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1.. self.width - 1);
        let mut new_y = rng.gen_range(1.. self.width - 1);
        // generate new coordinates again if old ones overlap with the snake body.
        // as snake grows in size, may be a cause to slow down the game
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1.. self.width -1);
            new_y = rng.gen_range(1..self.width -1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // updates snake after each frame of the game
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating()
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    // restarts the game after death of snake. 
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }

}

