mod ffi;
use std::ffi::CString;


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn c_binding() {
    //     let result = unsafe { test() };
    //     assert_eq!(result, 1);
    // }

    #[test]
    fn simple_window_test(){

        unsafe {
            let rust_string: String = String::from("C Test Game");
            let c_string: CString = CString::new(rust_string).expect("CString::new failed");
            let c_ptr = c_string.into_raw();

            ffi::create_game_window(c_ptr, 800, 600);

                // Create a sprite
            let sprite = ffi::create_sprite(100.0, 150.0, 50, 50, 255, 0, 0); // Red sprite

            // Create another sprite
            let sprite2 = ffi::create_sprite(200.0, 300.0, 60, 60, 0, 255, 0); // Green sprite

            // Main loop
            loop {
                if ffi::window_should_close() == 1 {
                    break;
                }
                ffi::clear_screen();

                // Render the sprite
                ffi::render_sprite(sprite);
                ffi::render_sprite(sprite2);

                // Update the game window
                ffi::update_game_window();
            }
        }
    }
}

