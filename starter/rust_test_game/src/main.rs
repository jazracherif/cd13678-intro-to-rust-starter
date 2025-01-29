use my_game_engine::game_ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK, C_STRING};

use std::{thread, time};
use std::ffi::CString;

mod game;
mod snake;
mod remote;
use snake::Snake;
use snake::Window;
use reqwest::Error;

const WINDOW_WIDTH  : i32 = 800;
const WINDOW_HEIGHT : i32 = 600;
const SPRIDE_SIDE   : i32 = 25;
const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);

#[tokio::main]
async fn main()  -> Result<(), Error>{
  
    let initial_sprite = remote::request_sprite().await;

    let title = C_STRING!("Snake Game");

    unsafe {
        game_ffi::create_game_window(title, WINDOW_WIDTH, WINDOW_HEIGHT);

        let snake = Snake::new(Window{width:WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE}, 
                                        initial_sprite.x, 
                                        initial_sprite.y,                                
                                        SPRIDE_SIDE, 
                                        SPRIDE_SIDE, 
                                        initial_sprite.r,
                                        initial_sprite.g,
                                        initial_sprite.b);
        let mut game = game::Game::create_snakes(vec![snake]);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
            game.render();            
        });
        game.stop();
        Ok(())
    }
}
