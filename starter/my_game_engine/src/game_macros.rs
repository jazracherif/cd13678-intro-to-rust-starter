

#[macro_export]
macro_rules! spawn_sprite {
    ($render:literal, $($x:expr),*) => {
            {
            let sprite = ffi::create_sprite( $($x),* );
            if $render {
                ffi::render_sprite(sprite);
            }
            sprite
        }
    };
}
