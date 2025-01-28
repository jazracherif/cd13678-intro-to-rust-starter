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
    speed: f32, // pixel per time tick
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

macro_rules! SPRITE_ATTR {
    ($sprite:ident, $attr:ident) => {
        (*$sprite).$attr
    };
}

macro_rules! SPRITE_X {
    ($sprite:ident ) => {
        SPRITE_ATTR!($sprite, x)
    };
}

macro_rules! SPRITE_Y {
    ($sprite:ident) => {
        SPRITE_ATTR!($sprite, y)
    };
}

// going beyoung the *left* boundary
// should take us to the right side of the window
macro_rules! GO_LEFT {
    ($sprite:ident, $window:expr, $speed:expr) => {
        if SPRITE_X!($sprite) < - $window.sprite_side as f32 {
            $window.width as f32
        } else {
            SPRITE_X!($sprite) - 1.0 * $speed 
        }
    };
}

macro_rules! GO_RIGHT {
    ($sprite:ident, $window:expr, $speed:expr) => {
        if SPRITE_X!($sprite) > $window.width as f32 { 
            - $window.sprite_side as f32 
        } else {
            SPRITE_X!($sprite) + 1.0 * $speed 
        }
    };
}

macro_rules! GO_UP {
    ($sprite:ident, $window:expr, $speed:expr) => {
        if SPRITE_Y!($sprite) == - $window.sprite_side as f32 { 
            $window.height as f32
        } else {
            SPRITE_Y!($sprite) - 1.0 * $speed 
        }
    };
}

macro_rules! GO_DOWN {
    ($sprite:ident, $window:expr, $speed:expr) => {
        if SPRITE_Y!($sprite) > $window.height as f32 { 
            - $window.sprite_side as f32
        } else {
            SPRITE_Y!($sprite) + 1.0 * $speed 
        }
    };
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
                    // let new_x = if SPRITE_X!(sprite) < -self.window.sprite_side as f32 { self.window.width as f32} else {SPRITE_X!(sprite)- 1.0 * self.speed };
                    let new_x = GO_LEFT!(sprite, self.window, self.speed);
                    DUPE_SPRITE!(sprite , new_x , SPRITE_Y!(sprite))
                },
                Direction::RIGHT => { 
                    let new_x = GO_RIGHT!(sprite, self.window, self.speed);
                    DUPE_SPRITE!(sprite , new_x, SPRITE_Y!(sprite) )
                },
                Direction::UP => { 
                    let new_y = GO_UP!(sprite, self.window, self.speed);
                    DUPE_SPRITE!(sprite , SPRITE_X!(sprite) , new_y )
                },
                Direction::DOWN => { 
                    let new_y = GO_DOWN!(sprite, self.window, self.speed);
                    DUPE_SPRITE!(sprite , SPRITE_X!(sprite) , new_y)
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


