//! main.rs
//! 
//! Main entry point for the Snake Game
//! 
//! This module 

use my_game_engine::game_ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK, C_STRING, TEXT_RENDER};

use std::{thread, time};
use std::ffi::CString;

mod game;
mod snake;
mod remote;
use snake::Snake;
use reqwest::Error;

use game_ffi::Window;

const WINDOW_WIDTH  : i32 = 800;
const WINDOW_HEIGHT : i32 = 600;
const SPRIDE_SIDE   : i32 = 25;
const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);
const GAME_OVER_LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(1000);


fn render_game_over(){
    static mut red: bool = true;
    let score_text = C_STRING!("!! GAME OVER !!");

    unsafe {
        match red {
            true => {TEXT_RENDER!(score_text, 350.0, 300.0, 500.0, 255.0, 0.0, 0.0); red=false;},
            false => {TEXT_RENDER!(score_text, 350.0, 300.0, 500.0, 0.0, 255.0, 0.0); red=true;},

        }
    }
}

#[tokio::main]
async fn main()  -> Result<(), Error>{
  
    let initial_sprite = remote::request_sprite().await;

    let title = C_STRING!("Snake Game");

    unsafe {
        game_ffi::create_game_window(title, WINDOW_WIDTH, WINDOW_HEIGHT);
    }

    let snake = Snake::new(Window{width:WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE}, 
                                        initial_sprite.x, 
                                        initial_sprite.y,                                
                                        SPRIDE_SIDE, 
                                        SPRIDE_SIDE, 
                                        0,
                                        255,
                                        0);

    let mut game = game::Game::create_snakes(vec![snake]);

    // Main loop
    START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
        if game.running() {
            game.render();
        } else {
            // break to game over loop
            break;
        }
    });

    // Game Over Loop
    START_WINDOW_AND_GAME_LOOP!(GAME_OVER_LOOP_SLEEP_MS, {
        render_game_over();
    });
            
    // cleanup
    if game.running() {
        game.stop();
    }
    Ok(())

}
