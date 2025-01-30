pub mod game_ffi;
pub mod game_macros;

use std::ffi::CString;
use std::{thread, time};


#[cfg(test)]
mod tests {
    use super::*;

    const LOOP_SLEEP_MS: time::Duration = time::Duration::from_millis(10);

    const WINDOW: game_ffi::Window = game_ffi::Window{width: 800, height: 600, sprite_side: 60};

    /// test_simple_game_loop: 
    /// Tests the basic game loop functionality. In this test, you should create_window
    /// (use 800 width, and 600 height), and loop until window_should_close is true while 
    /// calling your binding to update_game_window to catch the signal that the window has 
    /// been closed. In the game loop, you should also add a std::thread::sleep call to 
    /// some milliseconds to avoid overworking the CPU. Completing this test should spawn 
    /// a blank window that stays open until closed.
    #[test]
    #[ignore]
    fn test_simple_game_loop(){
        let title = C_STRING!("RUNNING test_simple_game_loop");

        unsafe { game_ffi::create_game_window(title, WINDOW.width, WINDOW.height); }

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {});
    }

    /// test_sprite_rendering: 
    /// Tests rendering a sprite on an open window with calls to your bindings over 
    /// create_sprite, and render_sprite. The success criteria for this test is to 
    /// have a functional window with a sprite rendered on it.
    #[test]
    #[ignore]
    fn test_sprite_rendering(){
        let title = C_STRING!("RUNNING test_sprite_rendering");

        unsafe {     
            game_ffi::create_game_window(title, WINDOW.width, WINDOW.height);
        }
        // Create a sprite
        SPAWN_SPRITE!(true, 100.0, 150.0, WINDOW.sprite_side, WINDOW.sprite_side, 255, 0, 0);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, {});   
        
    }

    // Same as test_sprite_rendering but flicker between two colors
    #[test]
    #[ignore]
    fn test_sprite_flicker(){
        let title = C_STRING!("RUNNING test_sprite_rendering");

        let mut red = true;

        unsafe { game_ffi::create_game_window(title, WINDOW.width, WINDOW.height); 
        }
        // Create a sprite
        let sprite = SPAWN_SPRITE!(true, 100.0, 150.0, WINDOW.sprite_side, WINDOW.sprite_side, 255, 0, 0);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS,
            {
                red = match red {
                    true =>  { CHANGE_SPRITE_COLOR!(sprite, 0, 255, 0); false},
                    false => { CHANGE_SPRITE_COLOR!(sprite, 255, 0, 0); true},
                };    
            }
        );
    }

    /// test_screen_clearing: 
    /// Tests clearing the screen. This is done by rendering a sprite on the window, 
    /// clearing the screen (with your bindings over clear_screen), and then rendering 
    /// another sprite. The success criteria for this test is to be able to visually 
    /// perceive the clearing of the screen. I rendered a red sprite for 2 seconds, then 
    /// cleared the screen and rendered a green sprite, and back to green afterwards in a loop
    ///  until the window closed.
    #[test]
    #[ignore]
    fn test_screen_clearing(){
        let title: *mut u8 = C_STRING!("RUNNING: test_screen_clearing");
        let switch_sprite_in_ms = time::Duration::from_millis(500);

        unsafe { game_ffi::create_game_window(title, WINDOW.width, WINDOW.height); }

        let sprite_red = SPAWN_SPRITE!(true, 100.0, 150.0, WINDOW.sprite_side, WINDOW.sprite_side, 255, 0, 0);
        let sprite_green = SPAWN_SPRITE!(false, 200.0, 300.0, WINDOW.sprite_side, WINDOW.sprite_side, 0, 255, 0);
        
        let mut red = true;
        let mut now =  time::Instant::now();

        // Main loop: switch between red and green 
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS,
            {
                if now.elapsed() >= switch_sprite_in_ms {
                    unsafe {
                        game_ffi::clear_screen();
                        red = match red {
                            true => {game_ffi::render_sprite(sprite_red); false }
                            false => {game_ffi::render_sprite(sprite_green); true }
                        };
                    }
                    now = time::Instant::now();
                }
            }
        );
    }
    

    /// test_key_presses: Tests handling key presses. This test should create a 
    /// window and register key press listeners. I set boolean variables on left 
    /// and right keypress and then closed the window when all were true. The
    /// success criteria for this test is to be able to visually perceive the key presses.
    #[test]
    #[ignore]
    fn test_key_presses() {
        let title: *mut u8 = C_STRING!("RUNNING: test_key_presses - [ PRESS LEFT + RIGHT]");

        unsafe { game_ffi::create_game_window(title, WINDOW.width, WINDOW.height); }
        // Create a sprite
        SPAWN_SPRITE!(true, 100.0, 150.0, WINDOW.sprite_side, WINDOW.sprite_side, 255, 0, 0);

        let mut key_left_pressed = false;
        let mut key_right_pressed = false;
        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS, 
            {
                ON_KEY_PRESS!(game_ffi::GLFW_KEY_LEFT, { key_left_pressed = true; });
                ON_KEY_PRESS!(game_ffi::GLFW_KEY_RIGHT, { key_right_pressed = true; });

                if key_left_pressed && key_right_pressed {
                    unsafe { game_ffi::clear_screen(); }
                    break;
                }
            }
        );
        
    }

    /// test_sprite_position_update: Tests updating the sprite position. 
    /// This test should create a window, render a sprite, and then update
    ///  the sprite's position (with your bindings to update_sprite_position).
    ///  The success criteria for this test is to be able to visually perceive 
    /// the sprite moving across the screen. Note: you might want to leverage 
    /// clear_screen to clear the screen between sprite updates.
    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let title: *mut u8 = C_STRING!("RUNNING: test_sprite_position_update - [MOVE AROUND]");

        unsafe { game_ffi::create_game_window(title, WINDOW.width, WINDOW.height); }

            // Create a sprite
        let sprite = SPAWN_SPRITE!(true, 100.0, 150.0, WINDOW.sprite_side, WINDOW.sprite_side, 255, 0, 0);

        // Main loop
        START_WINDOW_AND_GAME_LOOP!(LOOP_SLEEP_MS,
            {
                ON_KEY_PRESS!(game_ffi::GLFW_KEY_LEFT, {
                    let new_x = GO_LEFT!(sprite, WINDOW, 1.0);
                    MOVE_SPRITE!(true, true, sprite, new_x, SPRITE_Y!(sprite));  
                });

                ON_KEY_PRESS!(game_ffi::GLFW_KEY_RIGHT, {
                    let new_x = GO_RIGHT!(sprite, WINDOW, 1.0);
                    MOVE_SPRITE!(true, true, sprite, new_x, SPRITE_Y!(sprite));
                });

                ON_KEY_PRESS!(game_ffi::GLFW_KEY_UP, {
                    let new_y = GO_UP!(sprite, WINDOW, 1.0);
                    MOVE_SPRITE!(true, true, sprite, SPRITE_X!(sprite), new_y);
                });

                ON_KEY_PRESS!(game_ffi::GLFW_KEY_DOWN, {
                    let new_y = GO_DOWN!(sprite, WINDOW, 1.0);
                    MOVE_SPRITE!(true, true, sprite, SPRITE_X!(sprite), new_y);
                });
            }
        );
    }    
}


