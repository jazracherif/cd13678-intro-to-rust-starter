use serde::{ Deserialize, Serialize};
use serde_json::Result as serde_json_result;
use std::os::raw::{c_int};
use std::time;

use my_game_engine::game_ffi;
use my_game_engine::{START_WINDOW_AND_GAME_LOOP, TICK, C_STRING};

use crate::snake::{self, Snake};

use snake::Movement;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteData {
    pub width: i32,
    pub height: i32,
    pub x: f32,
    pub y: f32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

pub async fn call(url: &String) -> Result<String, reqwest::Error>{
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    
    Ok(body)
}

pub fn decode(body: String) -> serde_json_result<SpriteData>{

    let sprite: SpriteData = serde_json::from_str(body.as_str()).unwrap();

    Ok(sprite)
}

const SPRITE_SERVER_URL: &str = "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";

// TODO: handle case where the sprite is too dark to show against black background
pub async fn request_sprite() -> SpriteData  {
    println!("Grabbing initial Sprite Info from {}", SPRITE_SERVER_URL);

    let resp = call(&String::from(SPRITE_SERVER_URL)).await;

    let sprite = decode(resp.unwrap());

    println!("Received {:?}", sprite);

    sprite.unwrap()
}

const FOOD_UPDATE_EVERY : time::Duration = time::Duration::from_secs(1);
const FOOD_EXPIRES_IN : time::Duration = time::Duration::from_secs(20);

struct Food {
    sprite: *mut game_ffi::Sprite,
    expires: time::Instant,
}

pub struct Game{
    snakes: Vec<Snake>, // can be accessed by multiple threads
    food: Vec<Food>, // these will be populated by background app
    next_food_at: time::Instant
}

impl Game {
    pub fn create_all(snakes: Vec<Snake>, food: Vec<Food>) -> Game {
        Game{ snakes: snakes, food: food, next_food_at: time::Instant::now()}
    }
    pub fn create_snakes(snakes: Vec<Snake>) -> Game {
        Game{ snakes: snakes, food: Vec::new(), next_food_at: time::Instant::now()}
    }

    pub fn render(&mut self){
        unsafe {

            game_ffi::clear_screen();

            for snake in self.snakes.iter_mut() {
                snake.go();
                snake.render(); 
            }

            self.render_food();
        }

    }
    fn render_food(&self){
        unsafe {
            for food in self.food.iter() {
                game_ffi::render_sprite(food.sprite);
            }
        }
    }

    fn check_expired(){

    }
}