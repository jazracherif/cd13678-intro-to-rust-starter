use my_game_engine::game_ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK, C_STRING};

use std::{thread, time};
use std::ffi::CString;

mod snake;
use snake::Snake;
use snake::{Movement, Window};

const WINDOW_WIDTH  : i32 = 800;
const WINDOW_HEIGHT : i32 = 600;
const SPRIDE_SIDE   : i32 = 25;
const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);

fn main() {
    let title = C_STRING!("Snake Game");
    unsafe {
        game_ffi::create_game_window(title, WINDOW_WIDTH, WINDOW_HEIGHT);
        let mut snake = Snake::new(Window{width:WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE}, 
                                        100.0, 
                                        150.0, 
                                        SPRIDE_SIDE, 
                                        SPRIDE_SIDE, 
                                        255,  // red
                                        0,
                                         0);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
            game_ffi::clear_screen();

            snake.go();
            snake.render();
            
        });
    }
}
