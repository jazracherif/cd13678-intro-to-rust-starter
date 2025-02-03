//! snake module
//!
//! Instantiate and control a snake. The snake can be user controlled, or autonomous.
//! Expects a game to have already been instantiated.
//!
use core::cmp::PartialEq;
use rand::Rng;
use std::option::Option;
use std::{collections::VecDeque, u8};

use crate::game_ffi;
use game_ffi::Window;
use my_game_engine::{
    DUPE_SPRITE, GO_DOWN, GO_LEFT, GO_RIGHT, GO_UP, ON_KEY_PRESS, SPAWN_SPRITE, SPRITE_ATTR,
    SPRITE_X, SPRITE_Y,
};

const SNAKE_BODY_DISPLACEMENT_SPEED_PER_ITERATION: i32 = 3;
const INITIAL_SNAKE_GROWTH_SPEED: f32 = 1.0;
const SNAKE_GROWTH_RATE: f32 = 0.02;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq)]
pub enum SnakeKind {
    /// User controlls this snake with keyboard
    USER,
    // buddy user, doesn't die from eating bad food, mimics the user's snake
    BUDDY,
    // this snake randomly goes around to distract the user
    AUTONOMOUS,
}

pub struct GameSprite {
    pub sprite: *mut game_ffi::Sprite,
}

impl GameSprite {
    fn from_sprite(sprite: *mut game_ffi::Sprite) -> GameSprite {
        GameSprite { sprite: sprite }
    }

    fn render(&self) {
        unsafe {
            game_ffi::render_sprite(self.sprite);
        }
    }
}

pub struct Snake {
    /// the snake's body
    body: VecDeque<GameSprite>,
    /// the number of body parts to move at each step
    speed: i32,
    /// by how many pixels to move the head of the snake
    stride: f32,
    /// the direction of the snake's head
    direction: Direction,
    /// the game's window
    window: Window,
    /// whether this snake is a shadow buddy or not.
    pub kind: SnakeKind,
    /// Random generator helps with deciding the direction of the autonomous snakes
    rng: rand::rngs::ThreadRng,
}

pub trait SnakeMovement {
    /// Make the snake crawl without growth
    fn crawl(&mut self);

    /// Grow the snake
    fn grow(&mut self);
}

impl Snake {
    // Public Methods
    pub fn new(
        kind: SnakeKind,
        window: Window,
        x: f32,
        y: f32,
        width: i32,
        height: i32,
        r: i32,
        g: i32,
        b: i32,
    ) -> Snake {
        let snake_body_item =
            GameSprite::from_sprite(SPAWN_SPRITE!(false, x, y, width, height, r, g, b));
        Snake {
            kind: kind,
            direction: Direction::RIGHT,
            speed: SNAKE_BODY_DISPLACEMENT_SPEED_PER_ITERATION,
            stride: INITIAL_SNAKE_GROWTH_SPEED,
            body: VecDeque::from([snake_body_item]),
            window: window,
            rng: rand::rng(),
        }
    }

    pub fn render(&self) {
        for snake_body_item in self.body.iter() {
            snake_body_item.render();
        }
    }

    pub fn head(&self) -> Option<&GameSprite> {
        self.body.front()
    }

    pub fn is_owned_by_user(&self) -> bool {
        self.kind == SnakeKind::USER || self.kind == SnakeKind::BUDDY
    }

    pub fn dies_from_bad_food(&self) -> bool {
        self.kind == SnakeKind::USER
    }
    // Private Methods

    /// USER and BUDDY snakes are controlled manually through the keyboard, while autonomous snakes
    /// roam around the screen in random but smooth way
    fn update_direction(&mut self) {
        if self.kind == SnakeKind::AUTONOMOUS {
            let distr = rand::distr::Uniform::new_inclusive(0, 50).unwrap();
            let random: u8 = self.rng.sample(distr);
            match random {
                0..=46 => {} // 92% stay on the same course
                47 => {
                    self.direction = Direction::LEFT;
                }
                48 => {
                    self.direction = Direction::RIGHT;
                }
                49 => {
                    self.direction = Direction::UP;
                }
                50 => {
                    self.direction = Direction::DOWN;
                }
                41..=u8::MAX => {}
            };
            return;
        }

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

    /// Move the snake forward, delete the back of the snake if no growth is expected
    fn move_forward(&mut self, grow: bool) {
        for _ in 0..self.speed {
            let sprite: *mut game_ffi::Sprite = self.body.front().expect("Empty head").sprite;
            let new_head = match self.direction {
                Direction::LEFT => {
                    let new_x = GO_LEFT!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite, new_x, SPRITE_Y!(sprite))
                }
                Direction::RIGHT => {
                    let new_x = GO_RIGHT!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite, new_x, SPRITE_Y!(sprite))
                }
                Direction::UP => {
                    let new_y = GO_UP!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite, SPRITE_X!(sprite), new_y)
                }
                Direction::DOWN => {
                    let new_y = GO_DOWN!(sprite, self.window, self.stride);
                    DUPE_SPRITE!(sprite, SPRITE_X!(sprite), new_y)
                }
            };
            self.body.push_front(GameSprite::from_sprite(new_head));

            if !grow {
                self.body.pop_back();
            }
        }
    }
}

impl SnakeMovement for Snake {
    /// move head and delete the tail without rendering
    fn crawl(&mut self) {
        self.update_direction();

        self.move_forward(false);
    }

    /// expand the snake with a new head in the same direction
    fn grow(&mut self) {
        self.stride += SNAKE_GROWTH_RATE;

        self.move_forward(true);
        println!(
            "Snake size: {} - speed:{} - stride: {}",
            self.body.len(),
            self.speed,
            self.stride
        );
    }
}
