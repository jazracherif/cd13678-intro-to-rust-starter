use std::collections::VecDeque;

use crate::ffi;
use my_game_engine::{ON_KEY_PRESS, DUPE_SPRITE, MOVE_SPRITE, SPAWN_SPRITE};

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Window {
    pub width: i32,
    pub height: i32,
    pub sprite_side: i32,
}

pub struct Snake {
    body: VecDeque<*mut ffi::Sprite>,
    speed: f32,
    direction: Direction,
    window: Window,
}

pub trait Movement {
    // keep the object moving without change
    fn go(&mut self);

    // Grow the snake
    fn grow(&mut self, tail: *mut ffi::Sprite);

    // Shrink from the tail end
    fn shrink(&mut self);
}

impl Snake {
    // Public Methods
    pub fn new(window: Window, x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b:i32) -> Snake {
        let sprite: *mut ffi::Sprite;
        unsafe {
            sprite = SPAWN_SPRITE!(false, x, y, width, height, r, g, b);
        }
        Snake{ direction: Direction::RIGHT, speed: 1.0, body: VecDeque::from([ sprite ]), window: window }
    }

    pub fn render(&self){
        unsafe {
            for sprite in self.body.iter(){
                ffi::render_sprite(*sprite);
            }
        }
    }

    // Private Methods

    /// Check whether use changed the snake's direction, and return the new location of the head node
    fn update_direction(&mut self) {
        unsafe {
            ON_KEY_PRESS!(ffi::GLFW_KEY_LEFT, {
                self.direction = Direction::LEFT;
            });

            ON_KEY_PRESS!(ffi::GLFW_KEY_RIGHT, {
                self.direction = Direction::RIGHT;
            });

            ON_KEY_PRESS!(ffi::GLFW_KEY_UP, {
                self.direction = Direction::UP;
            });

            ON_KEY_PRESS!(ffi::GLFW_KEY_DOWN, {
                self.direction = Direction::DOWN;
            });
        }
    }

    fn create_new_head(&mut self) {
        let sprite = *self.body.front().expect("Empty head");

        unsafe {
            let new_head = match self.direction {
                Direction::LEFT => { 
                    let new_x = if (*sprite).x < -self.window.sprite_side as f32 { self.window.width as f32} else {(*sprite).x - 1.0 };              
                    DUPE_SPRITE!(sprite , new_x , (*sprite).y)
                },
                Direction::RIGHT => { 
                    let new_x = if (*sprite).x > self.window.width as f32 { -self.window.sprite_side as f32 } else {(*sprite).x + 1.0 };
                    DUPE_SPRITE!(sprite , new_x, (*sprite).y ) 
                },
                Direction::UP => { 
                    let new_y = if (*sprite).y == -self.window.sprite_side as f32 { self.window.height as f32 } else {(*sprite).y - 1.0 };
                    DUPE_SPRITE!(sprite , (*sprite).x , new_y ) 
                },
                Direction::DOWN => { 
                    let new_y = if (*sprite).y > self.window.height as f32 { -self.window.sprite_side as f32 } else {(*sprite).y + 1.0 };
                    DUPE_SPRITE!(sprite , (*sprite).x , new_y) 
                },
            };

            self.body.push_front(new_head);
        }        
    }

}

impl Movement for Snake {
    /// 
    fn go(&mut self) {
        self.update_direction();
        self.create_new_head();        
        self.body.pop_back();
    }

    fn grow(&mut self, new_item: *mut ffi::Sprite) {
        self.body.push_front(new_item);
    }

    fn shrink(&mut self) {
        self.body.pop_back();
    }
}


