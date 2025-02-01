use std::time;
use std::thread;
use std::sync::{Arc, Mutex};
use crossbeam_channel::unbounded;
use my_game_engine::C_STRING;
use std::ffi::CString;

// use rand::prelude::*; // DEBUG
const SPRIDE_SIDE   : i32 = 25; // TODO: merge with the one in main.rs

use crate::remote;
use crate::remote::SpriteData;
use crate::snake::{self, Snake};
use snake::Movement;

use my_game_engine::game_ffi;
use my_game_engine::{SPAWN_SPRITE, SPRITE_X, SPRITE_Y, SPRITE_ATTR, SPRITE_HEIGHT, SPRITE_WIDTH, TEXT_RENDER};

const FOOD_UPDATE_EVERY : time::Duration = time::Duration::from_millis(500);
const FOOD_EXPIRES_IN : time::Duration = time::Duration::from_secs(100);
use core::cmp::PartialEq;

#[derive(PartialEq, Clone)]
enum FoodType {
    Good,
    Bad
}

#[derive(PartialEq, Clone)]
pub struct Food {
    sprite: *mut game_ffi::Sprite,
    expires: time::Instant,
    food_type: FoodType
}


/// Check whether sprite 2 overlaps sprite 1 
macro_rules! CHECK_SPRITE_OVERLAP {
    ($s1:expr, $s2:expr) => {
        {
            let mut inside: bool = false;
            let x1 = SPRITE_X!($s1);
            let y1 = SPRITE_Y!($s1);
            let x2 = SPRITE_X!($s2);
            let y2 = SPRITE_Y!($s2);
            // gather all corners in sprite number 2
            let corners = vec![ (x2, y2), 
                                (x2 + SPRITE_WIDTH!($s2) as f32, y2), 
                                (x2, y2 + SPRITE_HEIGHT!($s2) as f32), 
                                (x2 + SPRITE_WIDTH!($s2) as f32, y2 + SPRITE_HEIGHT!($s2) as f32)];
            
            // check whether each corner in sprite 2 is contained in sprite 1
            for (x, y) in corners {
                if (x >= x1 && x <= x1 + SPRITE_WIDTH!($s1) as f32) && 
                    (y >= y1 &&  y < y1 + SPRITE_HEIGHT!($s1) as f32){
                        inside = true;
                        break;
                    }
            }
            inside
        }
    };
}

pub struct Game{
    snakes: Vec<Snake>, // can be accessed by multiple threads
    food: Vec<Food>, // these will be populated by background app
    last_food_fetched: time::Instant,
    running: Arc<Mutex<bool>>,
    channels: (crossbeam_channel::Sender<i32>, crossbeam_channel::Receiver<SpriteData>),
    score: i32,
}

impl Game {
    pub fn create_all(snakes: Vec<Snake>, food: Vec<Food>) -> Game {
        // setup background thread for network requests

        let (sender_main, receiver_remote) = unbounded(); // one way from main to background
        let (sender_remote, receiver_main) = unbounded(); // one way from backgroun to main

        let game = Game{ 
                            snakes: snakes, 
                            food: food, 
                            last_food_fetched: time::Instant::now(), 
                            running: Arc::new(Mutex::new(true)),
                            channels: (sender_main, receiver_main),
                            score: 0
                        };

        let thread_sender = sender_remote.clone();
        let thread_receiver = receiver_remote.clone();
        let running_clone = Arc::clone(&game.running);

        /* Create a background Thread for handling network request for sprite data.
           This thread communicates with the main thread through 2 messages queues, 
           one for receiving requests, one for sending back the data. Each request
           is handled via an async task, so that multiple network request may be 
           serviced at the same time.
        */
        thread::spawn( move || {
            // let mut rng = rand::rng(); // DEBUG
            // let nums: Vec<i32> = (1..400).collect(); // DEBUG
            
            let runtime = tokio::runtime::Runtime::new().unwrap();            

            loop {
                if *(running_clone.lock().unwrap()) == false {
                    // exit thread
                    println!("Game not running, Closing thread");
                    drop(thread_sender);
                    break;
                }
                // listen to request, spawn a new async task for each new req from main
                for _ in thread_receiver.iter(){
                    // let result: SpriteData = SpriteData{x: *nums.choose(&mut rng).unwrap() as f32, 
                    // y: *nums.choose(&mut rng).unwrap() as f32, width: 25, height:25, r:255, g:0, b:0 }; // DEBUG
                    let thread_sender_clone = thread_sender.clone();
                    runtime.spawn(runtime.spawn(async move {
                        let sprite_data = remote::request_sprite().await;
                        // send it back to the main thread
                        let _ = thread_sender_clone.send(sprite_data);
                    }));            
                }
                std::thread::sleep(time::Duration::from_millis(10));
            }
        });
    
        game
    }

    pub fn create_snakes(snakes: Vec<Snake>) -> Game {
        Game::create_all(snakes, Vec::new())
    }

    pub fn render(&mut self){
        unsafe {
            game_ffi::clear_screen();
        }
        self.render_snakes();

        self.render_food();

        // Render Score
        self.render_score();
    }

    fn render_score(&self) {
        let score = self.score.to_string();
        let score_text = C_STRING!(format!("score={score}"));
        TEXT_RENDER!(score_text, 0.0, 20.0, 100.0, 255.0, 0.0, 0.0);
    }

    pub fn stop(&mut self){
        println!("STOP the game");
        let mut running = self.running.lock().unwrap();
        *running = false;
    }

    pub fn running(&self) -> bool{
        *self.running.lock().unwrap()
    }


    fn render_snakes(&mut self){
        let mut dead_snake: Option<&*mut game_ffi::Sprite> = None;
        {
            for snake in self.snakes.iter_mut() {
                snake.crawl();
                // check if we encountered food
                let food_consumed: Vec<Food> =  match snake.head() {
                    Some(head) => {     
                        self.food.iter().cloned()
                            .filter(|food| CHECK_SPRITE_OVERLAP!(food.sprite, *head)).collect::<Vec<Food>>()                              
                    },
                    None => { vec![] }
                };
                if !food_consumed.is_empty() {
                    if !snake.shadow() && food_consumed.iter().filter(|food| food.food_type == FoodType::Bad).count() != 0  {
                        // bad food eaten, die!
                        dead_snake = Some(snake.head().expect("Head not found!"));
                        break;
                    }

                    self.score += food_consumed.len() as i32;
                    snake.grow();

                    // remove food items
                    self.food.retain(|food| !food_consumed.contains(&food));
                    println!("food eaten! remaining food {}", self.food.len());
                }
                snake.render(); 
            }
        }
        
        match dead_snake {
            Some(&head_sprite) => {
                self.stop();
                SPAWN_SPRITE!(true, 
                            SPRITE_X!(head_sprite), 
                            SPRITE_Y!(head_sprite),
                            SPRITE_WIDTH!(head_sprite), 
                            SPRITE_HEIGHT!(head_sprite), 
                            250,  
                            255,  
                            255);
            },
            None => {}
        };
    }

    fn render_food(&mut self){
        // cleanup
        self.remove_expired_food();

        // is there new food
        let mut new_food: Vec<Food> = Vec::new();
        self.check_new_food_downloaded(&mut new_food);
        self.food.append(&mut new_food);
         
        // request new food
        if self.last_food_fetched.elapsed() > FOOD_UPDATE_EVERY {
            self.request_new_food();
            self.last_food_fetched = time::Instant::now();
        }

        //render
        unsafe {
            self.food.iter().for_each( move |food| {
                game_ffi::render_sprite(food.sprite);   
            });
        }
    }

    // Check whether any food has expired and remove it
    fn remove_expired_food(&mut self){
        self.food.retain(|x| x.expires.elapsed() < FOOD_EXPIRES_IN);
    }

    fn check_new_food_downloaded(&self, new_food: & mut Vec<Food>) {
        let receiver = &self.channels.1;

        if !receiver.is_empty(){
            receiver.try_iter().for_each(move |mut sprite_data| {
                let mut food_type = FoodType::Good;
                if sprite_data.r > 150 {
                    sprite_data.r = 255;
                    sprite_data.g = 0;
                    sprite_data.b = 0;
                    food_type = FoodType::Bad;
                }

                new_food.push(
                    Food{sprite: SPAWN_SPRITE!(false, sprite_data.x, sprite_data.y, 
                        SPRIDE_SIDE, SPRIDE_SIDE, sprite_data.r, sprite_data.g, sprite_data.b),
                        food_type,
                        expires: time::Instant::now()});
                });
            
        }
    }

    fn request_new_food(&self){
        println!("Request 1 more food item");
        let sender = &self.channels.0;
        let _ = sender.send(1);
    }
}