use my_game_engine::ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK};

use std::ffi::CString;
use std::{thread, time};

mod snake;
use snake::Snake;
use snake::{Movement, Window};

const WINDOW_WIDTH  : i32 = 800;
const WINDOW_HEIGHT : i32 = 600;
const SPRIDE_SIDE   : i32 = 25;
const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);

fn main() {
    let rust_string: String = String::from("RUNNING test_simple_game_loop");
    let c_string: CString = CString::new(rust_string).expect("CString::new failed");
    let c_ptr = c_string.into_raw();

    unsafe {

        ffi::create_game_window(c_ptr, WINDOW_WIDTH, WINDOW_HEIGHT);
        let mut snake = Snake::new(Window{width:WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE}, 
                                        100.0, 
                                        150.0, 
                                        SPRIDE_SIDE, 
                                        SPRIDE_SIDE, 
                                        255, 
                                        0,
                                         0);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
            ffi::clear_screen();

            snake.go();
            snake.render();
            
        });
    }
}
