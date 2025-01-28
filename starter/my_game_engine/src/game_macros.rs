
/// Create a sprite and rended it on the screen
#[macro_export]
macro_rules! SPAWN_SPRITE {
    ($render:literal, $($x:expr),*) => {
        {
            let sprite = game_ffi::create_sprite( $($x),* );
            if $render {
                game_ffi::render_sprite(sprite);
            }
            sprite
        }
    };
}

#[macro_export]
macro_rules! DUPE_SPRITE {
    ($sprite:ident, $x:expr, $y:expr) => {
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
/// and then rendering the sprite
#[macro_export]
macro_rules! MOVE_SPRITE {
    ($clear:literal, $sprite:ident, $new_x:expr, $new_y:expr) => {
        {
            if $clear {
                game_ffi::clear_screen();
            }
            game_ffi::update_sprite_position($sprite,  $new_x, $new_y);
            game_ffi::render_sprite($sprite);
        }
    };
}

#[macro_export]
macro_rules! TICK {
    ($sleepms:expr) => {
        // Update the game window
        game_ffi::update_game_window();
        thread::sleep($sleepms);
    };
}

#[macro_export]
macro_rules! ON_KEY_PRESS {
    ($key:expr, $block:block) => {
        if game_ffi::get_key(game_ffi::get_window(), $key) == game_ffi::GLFW_PRESS {
            $block
        }
    };
}

#[macro_export]
macro_rules! CHANGE_SPRITE_COLOR {
    ($sprite:ident, $r:literal, $g:literal, $b:literal) => {
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
        loop {
            if game_ffi::window_should_close() == 1 {
                break;
            }

            $loop_block

            TICK!($sleepms);
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
    ($sprite:ident, $attr:ident) => {
        (*$sprite).$attr
    };
}

#[macro_export]
macro_rules! SPRITE_X {
    ($sprite:ident ) => {
        SPRITE_ATTR!($sprite, x)
    };
}

#[macro_export]
macro_rules! SPRITE_Y {
    ($sprite:ident) => {
        SPRITE_ATTR!($sprite, y)
    };
}