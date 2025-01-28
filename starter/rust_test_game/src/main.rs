use my_game_engine::ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK, MOVE_SPRITE};

use std::ffi::CString;
use std::{thread, time};

mod snake;
use snake::Snake;
use snake::{Movement, Direction, Window};

fn main() {
    let rust_string: String = String::from("RUNNING test_simple_game_loop");
    let c_string: CString = CString::new(rust_string).expect("CString::new failed");
    let c_ptr = c_string.into_raw();

    const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);

    unsafe {

        ffi::create_game_window(c_ptr, 800, 600);
        let mut snake = Snake::new(Window{width:800, height:600, sprite_side:10}, 
            100.0, 150.0, 10, 10, 255, 0, 0);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
            ffi::clear_screen();

            snake.go();
            snake.render();
            
        });
    }
}
