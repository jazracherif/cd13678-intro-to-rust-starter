//! Snake
//!
//! The Snake Game consists of a user controlled snake moving around a window frame
//! looking for food to eate. The goal is to score as many point as possible by eating good 
//! food. Bad food (color RED) eaten results in a game over.
//! 
//! As currently setup, the user's controlled snake is accompanied by a buddy snake, which
//! moves in the same direction and can also eat food. This snake doesn't die when eating bad
//! food and can thus be used as a helper. A third kind of snake is deployed that moves autonomously
//! across the window, creating a bit of distraction to the user. This snake also doesn't die.
//! 
//! Games can be restarted when a snake dies by pressing the `space` bar. A top left scrore box
//! shows the current score from eating food, counted those eaten by both the user and the buddy
//! snake

use my_game_engine::game_ffi;
use my_game_engine::{C_STRING, ON_KEY_PRESS, START_WINDOW_AND_GAME_LOOP, TEXT_RENDER, TICK, CREATE_GAME};
use remote::SpriteData;

use std::ffi::CString;
use std::{thread, time};

mod game;
use game::Game;

mod snake;
use snake::Snake;
use snake::SnakeKind;

mod remote;
use reqwest::Error;

use game_ffi::Window;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const SPRIDE_SIDE: i32 = 25;
const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);
const GAME_OVER_FLASH_EVERY_MS: time::Duration = time::Duration::from_millis(1000);

#[doc(hidden)]
fn render_game_over_message() {
    static mut RED: bool = true;
    let score_text = C_STRING!("!! GAME OVER !! (space to restart)");

    unsafe {
        match RED {
            true => {
                TEXT_RENDER!(score_text, 250.0, 300.0, 500.0, 255.0, 0.0, 0.0);
                RED = false;
            }
            false => {
                TEXT_RENDER!(score_text, 250.0, 300.0, 500.0, 0.0, 255.0, 0.0);
                RED = true;
            }
        }
    }
}

/// Main loop where all game events are handled
fn game_main_loop(game: &mut Game) {
    START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {
        if game.running() {
            game.render();
        } else {
            // break to game over loop
            break;
        }
    });
}

/// Handle the game over loop to allow restart
fn game_over_loop() -> bool {
    let mut restart: bool = false;
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

/// Create several snakes that will be used in the game
fn create_snakes(snakes: &mut Vec<Snake>, initial_sprite: &SpriteData) {
    // Create soome snakes
    let user_snake = Snake::new(
        SnakeKind::USER,
        Window {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            sprite_side: SPRIDE_SIDE,
        },
        initial_sprite.x,
        initial_sprite.y,
        SPRIDE_SIDE,
        SPRIDE_SIDE,
        0,
        255,
        0,
    );

    let buddy_snake = Snake::new(
        SnakeKind::BUDDY,
        Window {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            sprite_side: SPRIDE_SIDE,
        },
        initial_sprite.x + 25.0,
        initial_sprite.y + 25.0,
        SPRIDE_SIDE,
        SPRIDE_SIDE,
        25,
        25,
        25,
    );

    let autonomous_snake = Snake::new(
        SnakeKind::AUTONOMOUS,
        Window {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            sprite_side: SPRIDE_SIDE,
        },
        initial_sprite.x + 25.0,
        initial_sprite.y + 25.0,
        SPRIDE_SIDE,
        SPRIDE_SIDE,
        50,
        25,
        128,
    );

    snakes.push(user_snake);
    snakes.push(buddy_snake);
    snakes.push(autonomous_snake);
}

/// Main entrypoint for the program.
///  
/// Launches the game loop as well as the game over
/// loop and takes care of allowing users to restart the game if they lost.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let initial_sprite = remote::request_sprite().await;

    CREATE_GAME!(C_STRING!("Snake Game"), WINDOW_WIDTH, WINDOW_HEIGHT);

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
            break;
        }
    }

    Ok(())
}
