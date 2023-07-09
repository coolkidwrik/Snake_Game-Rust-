// linked lists from the standard library
use std::collections:: LinkedList;
// context and graphics buffer, similar to in draw.rs
use piston_window:: {Context, G2d};
// color, similar to in draw.rs
use piston_window::types::Color;

// importing public draw_block function from draw.rs
use crate::draw::draw_block;

// color of the snake in game
const SNAKE_COLOR: Color = [0.0, 0.5, 0.1, 1.0];

// enumeration for direction
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

// definition for direction object to be used in the code
impl Direction {

    //takes in a reference to self
    // returns the opposite direction to input
    pub fn opposite(&self) -> Direction {
        // match acts like an if/else statement
        // matches some input to some output
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}


// definition for a block
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

//fields for snake object
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

// methods for snake object
impl Snake {

    // constructor for snake object
    // takes in a starting position for the tail of the snake
    // initializes a linked list for the body of the snake
    // adds three blocks, making a snake of length three
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x+2, y });

        body.push_back(Block { x: x+1, y });

        body.push_back(Block { x, y });

        Snake { direction: Direction::Right, body, tail: None}
    }

    // renders the snake on the screen
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
    
    // returns a tuple with the x and y block coordinates of the snake
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }
    
    // updates the snake by moving it forward.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        
        // if a direction is provided, set the direction of the snake to the one provided
        // else do nothing
        match dir {
            Some(d) => self.direction = d,
            None => ()
        }

        // position of the head before the update
        let (last_x, last_y): (i32, i32) = self.head_position();
        
        // creates a new block to become the new head of the snake
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block { 
                x: last_x, 
                y: last_y + 1 
            },
            Direction::Left => Block { 
                x: last_x -1, 
                y: last_y 
            },
            Direction::Right => Block { 
                x: last_x + 1, 
                y: last_y 
            },
        };

        // adds the head
        self.body.push_front(new_block);
        // removes last block from the snake body
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    
    }
    
    // returns the direction of the snake head
    pub fn head_direction(&self) -> Direction {
        return self.direction;
    }

    // returns the updated position of the head given the direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y -1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x -1, head_y),
            Direction::Right => (head_x + 1, head_y)
        }
    }

    // when snake eats a fruit, tail is restored for the next frame
    // tail block stored in self.tail = some(removed_block) is pushed back into the body
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // returns true, if snake overlaps with itself
    // else false
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return  false;
    }



}

