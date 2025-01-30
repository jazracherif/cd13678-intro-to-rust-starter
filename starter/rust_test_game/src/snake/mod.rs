use std::collections::VecDeque;
use std::option::Option;

use crate::game_ffi;
use my_game_engine::{ON_KEY_PRESS, DUPE_SPRITE, SPAWN_SPRITE, SPRITE_X, SPRITE_Y, SPRITE_ATTR,
    GO_LEFT, GO_RIGHT, GO_UP, GO_DOWN };

use game_ffi::Window;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Snake {
    body: VecDeque<*mut game_ffi::Sprite>,
    
    pub speed: i32, // speed at which to move the snake. number of sprite moves per game loop
    stride: f32, // by how much to move the head of the snake

    direction: Direction,
    window: Window,
}

pub trait Movement {
    // keep the object moving without change
    fn crawl(&mut self);

    // Grow the snake
    fn grow(&mut self);

    // Shrink from the tail end
    fn shrink(&mut self);
}


impl Snake {
    // Public Methods
    pub fn new(window: Window, x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b:i32) -> Snake {
        let sprite: *mut game_ffi::Sprite;
        unsafe {
            sprite = SPAWN_SPRITE!(false, x, y, width, height, r, g, b);
        }
        Snake{ direction: Direction::RIGHT, speed: 1, stride: 2 as f32, body: VecDeque::from([ sprite ]), window: window }
    }

    pub fn render(&self){
        unsafe {
            for sprite in self.body.iter(){
                // println!("x:{} y:{}", SPRITE_X!(*sprite), SPRITE_Y!(*sprite));
                game_ffi::render_sprite(*sprite);
            }
        }
    }

    pub fn head(&self) -> Option< &*mut game_ffi::Sprite>{
        self.body.front()
    }

    // Private Methods

    /// Check whether use changed the snake's direction, and return the new location of the head node
    fn update_direction(&mut self) {
        unsafe {
            ON_KEY_PRESS!(game_ffi::GLFW_KEY_LEFT, {
                self.direction = Direction::LEFT;
            });

            ON_KEY_PRESS!(game_ffi::GLFW_KEY_RIGHT, {
                self.direction = Direction::RIGHT;
            });

            ON_KEY_PRESS!(game_ffi::GLFW_KEY_UP, {
                self.direction = Direction::UP;
            });

            ON_KEY_PRESS!(game_ffi::GLFW_KEY_DOWN, {
                self.direction = Direction::DOWN;
            });
        }
    }

    /// create and append a new head to the snake
    fn move_snake(&mut self, grow: bool) {

        for _ in 0..2 as i32 {
            let sprite: *mut game_ffi::Sprite = *self.body.front().expect("Empty head");
            let new_head = match self.direction {
                Direction::LEFT => { 
                    // let new_x = if SPRITE_X!(sprite) < -self.window.sprite_side as f32 { self.window.width as f32} else {SPRITE_X!(sprite)- 1.0 * self.speed };
                    let new_x = GO_LEFT!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite , new_x , SPRITE_Y!(sprite))
                },
                Direction::RIGHT => { 
                    let new_x = GO_RIGHT!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite , new_x, SPRITE_Y!(sprite) )
                },
                Direction::UP => { 
                    let new_y = GO_UP!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite , SPRITE_X!(sprite) , new_y )
                },
                Direction::DOWN => { 
                    let new_y = GO_DOWN!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite , SPRITE_X!(sprite) , new_y)
                },
            };   
            self.body.push_front(new_head);         
        
        if !grow {
            // self.body.drain((self.body.len() - self.speed as usize)..);
            self.body.pop_back();
        }
        }
    }

}

impl Movement for Snake {
    /// move head and delete the tail without rendering
    fn crawl(&mut self) {
        self.update_direction();
        self.move_snake(false);        
    }

    /// expand the snake with a new head in the same direction
    fn grow(&mut self) {
        // self.stride -= 0.01 * self.stride as f32;
        self.stride += 0.02;

        // self.speed += 1;
        self.move_snake(true);
        println!("Snake size: {} - speed:{} - stride: {}",self.body.len(), self.speed, self.stride);    
    }

    fn shrink(&mut self) {
        self.body.pop_back();
    }
}


