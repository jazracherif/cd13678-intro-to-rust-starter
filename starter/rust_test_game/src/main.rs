//! main.rs
//! 
//! Main entry point for the Snake Game
//! 
//! 

use my_game_engine::game_ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK, C_STRING, TEXT_RENDER, ON_KEY_PRESS};
use remote::SpriteData;

use std::{thread, time};
use std::ffi::CString;

mod game;
use game::Game;

mod snake;
use snake::SnakeKind;
use snake::Snake;

mod remote;
use reqwest::Error;

use game_ffi::Window;

const WINDOW_WIDTH  : i32 = 800;
const WINDOW_HEIGHT : i32 = 600;
const SPRIDE_SIDE   : i32 = 25;
const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);
const GAME_OVER_FLASH_EVERY_MS: time::Duration = time::Duration::from_millis(1000);


fn render_game_over_message(){
    static mut RED: bool = true;
    let score_text = C_STRING!("!! GAME OVER !! (space to restart)");

    unsafe {
        match RED {
            true => {TEXT_RENDER!(score_text, 250.0, 300.0, 500.0, 255.0, 0.0, 0.0); RED = false;},
            false => {TEXT_RENDER!(score_text, 250.0, 300.0, 500.0, 0.0, 255.0, 0.0); RED = true;},

        }
    }
}

fn game_main_loop(game: &mut Game){
    START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
        if game.running() {
            game.render();
        } else {
            // break to game over loop
            break;
        }
    });
}

fn game_over_loop() -> bool {
    let mut restart:bool = false;
    let mut time = time::Instant::now();

    render_game_over_message();

    START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
        if time.elapsed() >= GAME_OVER_FLASH_EVERY_MS {
            render_game_over_message();
            time = time::Instant::now();
        }

        ON_KEY_PRESS!(game_ffi::GLFW_KEY_SPACE, {
            // restart the game 
            restart = true;
            break;
        });
    });

    return restart;
}

fn create_snakes(snakes: &mut Vec<Snake>, initial_sprite: &SpriteData){
    // Create soome snakes
    let user_snake = Snake::new(
        SnakeKind::USER, 
            Window{width:WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE}, 
            initial_sprite.x,
            initial_sprite.y,
            SPRIDE_SIDE, 
            SPRIDE_SIDE,
            0,
            255,
            0);

    let buddy_snake = Snake::new(
        SnakeKind::BUDDY, 
            Window{width: WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE}, 
            initial_sprite.x + 25.0,
            initial_sprite.y + 25.0,
            SPRIDE_SIDE,
            SPRIDE_SIDE,
            25,
            25,
            25);

    let autonomous_snake = Snake::new(
        SnakeKind::AUTONOMOUS,
            Window{width: WINDOW_WIDTH, height: WINDOW_HEIGHT, sprite_side: SPRIDE_SIDE},
            initial_sprite.x + 25.0, 
            initial_sprite.y + 25.0,
            SPRIDE_SIDE,
            SPRIDE_SIDE,
            50,
            25,
            128);

    snakes.push(user_snake);
    snakes.push(buddy_snake);
    snakes.push(autonomous_snake);

}

#[tokio::main]
async fn main()  -> Result<(), Error>{
  
    let initial_sprite = remote::request_sprite().await;

    let title = C_STRING!("Snake Game");

    unsafe {
        game_ffi::create_game_window(title, WINDOW_WIDTH, WINDOW_HEIGHT);
    }

    loop { 
        println!("NEW GAME!");
        let mut snakes: Vec<Snake> = vec![];
        create_snakes(&mut snakes, &initial_sprite);
        
        // Create the game                                 
        let mut game = game::Game::new(snakes, vec![]);

        // Main Game loop. returns when the games ends
        game_main_loop(&mut game);

        // Game Over Loop
        if !game_over_loop() {
            if game.running() {
                // cleanup
                game.stop();
            }
            break;
        }
    }

    Ok(())

}
