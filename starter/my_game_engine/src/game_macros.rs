
/// Create a sprite and rended it on the screen
#[macro_export]
macro_rules! SPAWN_SPRITE {
    ($render:literal, $($x:expr),*) => {
        {
            let sprite: *mut game_ffi::Sprite;
            unsafe {
                sprite = game_ffi::create_sprite( $($x),* );
                if $render {
                    game_ffi::render_sprite(sprite);
                }
            }
            sprite
        }
    };
}

#[macro_export]
macro_rules! DUPE_SPRITE {
    ($sprite:expr, $x:expr, $y:expr) => {
        { 
            SPAWN_SPRITE!(false, 
                $x, 
                $y,
                (*$sprite).width, 
                (*$sprite).height, 
                (*$sprite).color[0],  
                (*$sprite).color[1],  
                (*$sprite).color[2]) 
        }
    };
}
/// move a sprite to a new position, potentially clearing screen first
/// and then rendering the sprite. This will render the sprite. Use
/// DUPE_SPRITE if you simple want to duplicate a sprite at a new location
#[macro_export]
macro_rules! MOVE_SPRITE {
    ($clear_before:literal, $render_after:literal, $sprite:expr, $new_x:expr, $new_y:expr) => {
        {
            unsafe {
                if $clear_before {
                    game_ffi::clear_screen();
                }
                game_ffi::update_sprite_position($sprite,  $new_x, $new_y);

                if $render_after {
                    game_ffi::render_sprite($sprite);
                }
            }            
        }
    };
}

#[macro_export]
macro_rules! TICK {
    ($sleepms:expr) => {
        {
            // Update the game window
            unsafe {
                game_ffi::update_game_window();
            }
            thread::sleep($sleepms);
        } 
    };
}

#[macro_export]
macro_rules! ON_KEY_PRESS {
    ($key:expr, $block:block) => {
        {
            let key_state: i32;
            unsafe {
                key_state = game_ffi::get_key(game_ffi::get_window(), $key);
            }

            if key_state == game_ffi::GLFW_PRESS {
                $block
            }
        }
    };
}

#[macro_export]
macro_rules! CHANGE_SPRITE_COLOR {
    ($sprite:expr, $r:literal, $g:literal, $b:literal) => {
        {
            let sprite = SPAWN_SPRITE!(true, 
                (*$sprite).x, 
                (*$sprite).y, 
                (*$sprite).width, 
                (*$sprite).height,
                $r, 
                $g, 
                $b);

            sprite
        }
    };
}

#[macro_export]
macro_rules! START_WINDOW_AND_GAME_LOOP {
    ($sleepms:expr, $loop_block:block) => {
        {
            loop {
                let should_close: i32;
                unsafe { should_close = game_ffi::window_should_close(); }

                if should_close == 1 {
                    break;
                }
    
                $loop_block
    
                TICK!($sleepms);
            }
        }
        
    };
}


#[macro_export]
macro_rules! C_STRING {
    ($string_expr:expr) => {
        {
            CString::new(String::from($string_expr)).expect("CString::new failed").into_raw()
        }
    };
}


#[macro_export]
macro_rules! SPRITE_ATTR {
    ($sprite:expr, $attr:ident) => {
        {
            unsafe {
                (*$sprite).$attr
            }
        }
    };
}

#[macro_export]
macro_rules! SPRITE_X {
    ($sprite:expr ) => {
        {
            SPRITE_ATTR!($sprite, x)
        }
    };
}

#[macro_export]
macro_rules! SPRITE_Y {
    ($sprite:expr) => {
        {
            SPRITE_ATTR!($sprite, y)
        }
    };
}

#[macro_export]
macro_rules! SPRITE_WIDTH {
    ($sprite:expr) => {
        {
            SPRITE_ATTR!($sprite, width)
        }
    };
}

#[macro_export]
macro_rules! SPRITE_HEIGHT {
    ($sprite:expr) => {
        {
            SPRITE_ATTR!($sprite, height)
        }
    };
}

#[macro_export]
macro_rules! GO_LEFT {
    ($sprite:expr, $window:expr, $speed:expr) => {
        if SPRITE_X!($sprite) < - $window.sprite_side as f32 {
            $window.width as f32
        } else {
            SPRITE_X!($sprite) - 1.0 * $speed 
        }
    };
}

#[macro_export]
macro_rules! GO_RIGHT {
    ($sprite:expr, $window:expr, $speed:expr) => {
        if SPRITE_X!($sprite) > $window.width as f32 { 
            - $window.sprite_side as f32 
        } else {
            SPRITE_X!($sprite) + 1.0 * $speed 
        }
    };
}

#[macro_export]
macro_rules! GO_UP {
    ($sprite:expr, $window:expr, $speed:expr) => {
        if SPRITE_Y!($sprite) < - $window.sprite_side as f32 { 
            $window.height as f32
        } else {
            SPRITE_Y!($sprite) - 1.0 * $speed 
        }
    };
}

#[macro_export]
macro_rules! GO_DOWN {
    ($sprite:expr, $window:expr, $speed:expr) => {
        if SPRITE_Y!($sprite) > $window.height as f32 { 
            - $window.sprite_side as f32
        } else {
            SPRITE_Y!($sprite) + 1.0 * $speed 
        }
    };
}

#[macro_export]
macro_rules! TEXT_RENDER {
    ($text:ident, $x:expr, $y:expr, $scale:expr, $r:expr, $g:expr, $b:expr) => {
        {
            unsafe { game_ffi::renderText($text, $x, $y, $scale, $r, $g, $b); }
        }
    };
}