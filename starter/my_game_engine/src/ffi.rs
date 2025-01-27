use std::os::raw::{c_char, c_int};
use std::ffi::c_void;

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;

pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

#[repr(C)]
pub struct Sprite {
    width: i32,
    height: i32,
    color: [c_int; 3], // RGB color
    pub x: f32,
    pub y: f32 // Position
}


extern "C" {
pub fn create_game_window(title:  *const c_char, width: i32, height: i32);

// Function to create a sprite
pub fn create_sprite(x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b:i32) -> *mut Sprite;

// Function to render a sprite
pub fn render_sprite(sprite: *mut Sprite);

// Function to update a sprite position
pub fn update_sprite_position(sprite: *mut Sprite,  x: f32, y: f32);

// Function to update the game window
pub fn update_game_window();

// Function to clear the screen
pub fn clear_screen();

// Function to check if the window should close
pub fn window_should_close() -> i32;

// // Function to get key state
pub fn get_key(window: *mut c_void, key: i32) -> i32;

// // Function to get the window pointer
pub fn get_window() -> *mut c_void ;

}
