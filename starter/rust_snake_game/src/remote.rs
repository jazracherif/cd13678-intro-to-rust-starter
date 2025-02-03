//! remote.rs
//! 
//! This module takes care of remote communication for fetching sprite info from a server

use serde::{Deserialize, Serialize};
use serde_json::Result as serde_json_result;

const SPRITE_SERVER_URL: &str =
    "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";

/// Data returned by the server will be turned into a SpriteData and served to the calling 
/// method
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

async fn call(url: &String) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;

    Ok(body)
}

fn decode(body: String) -> serde_json_result<SpriteData> {
    let sprite: SpriteData = serde_json::from_str(body.as_str()).unwrap();

    Ok(sprite)
}

// TODO: handle case where the sprite is too dark to show against black background
/// request a sprite from a predefined remote url
pub async fn request_sprite() -> SpriteData {
    let resp = call(&String::from(SPRITE_SERVER_URL)).await;
    let sprite = decode(resp.unwrap());

    sprite.unwrap()
}
