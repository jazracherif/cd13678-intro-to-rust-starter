use std::os::raw::{c_char, c_int};

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

#[repr(C)]
pub struct Sprite {
    width: usize,
    height: usize,
    color: [c_int; 3], // RGB color
    x: f64,
    y: f64 // Position
}

extern "C" {
pub fn create_game_window(title:  *const c_char, width: usize, height: usize);

// Function to create a sprite
pub fn create_sprite(x: f64, y: f64, width: usize, height: usize, r: usize, g: usize, b:usize) -> *mut Sprite;

// Function to render a sprite
pub fn render_sprite(sprite: *mut Sprite);

// Function to update a sprite position
pub fn update_sprite_position(sprite: *mut Sprite,  x: f64, y: f64);

// Function to update the game window
pub fn update_game_window();

// Function to clear the screen
pub fn clear_screen();

// Function to check if the window should close
pub fn window_should_close() -> usize;

// // Function to get key state
// fn get_key(GLFWwindow* window, int key) -> usize;

// // Function to get the window pointer
// GLFWwindow* get_window();

}
