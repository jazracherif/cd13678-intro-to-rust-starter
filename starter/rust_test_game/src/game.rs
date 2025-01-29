use std::time;
use std::thread;
use std::sync::{Arc, Mutex};

use rand::prelude::*;


use my_game_engine::game_ffi;

use crate::remote;
use crate::remote::SpriteData;
use crate::snake::{self, Snake};
use snake::Movement;
use my_game_engine::SPAWN_SPRITE;

use crossbeam_channel::unbounded;

const FOOD_UPDATE_EVERY : time::Duration = time::Duration::from_secs(2);
const FOOD_EXPIRES_IN : time::Duration = time::Duration::from_secs(10);

pub struct Food {
    sprite: *mut game_ffi::Sprite,
    expires: time::Instant,
}

pub struct Game{
    snakes: Vec<Snake>, // can be accessed by multiple threads
    food: Vec<Food>, // these will be populated by background app
    last_food_fetched: time::Instant,
    running: Arc<Mutex<bool>>,
    channels: (crossbeam_channel::Sender<i32>, crossbeam_channel::Receiver<SpriteData>)
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
                            channels: (sender_main, receiver_main)
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
            // let mut rng = rand::rng();
            // let nums: Vec<i32> = (1..400).collect();
            
            let runtime = tokio::runtime::Runtime::new().unwrap();            

            loop {
                if *(running_clone.lock().unwrap()) == false {
                    // exit thread
                    println!("Game not running, Closing thread");
                    drop(thread_sender);
                    break;
                }
                // listen to request, spawn a new async task for each new req from main
                for reqid in thread_receiver.iter(){
                    // let result: SpriteData = SpriteData{x: *nums.choose(&mut rng).unwrap() as f32, y: *nums.choose(&mut rng).unwrap() as f32, width: 25, height:25, r:255, g:0, b:0 };
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
            
            self.render_food();

            self.render_snakes();
        }
    }

    pub fn stop(&mut self){
        self.running = Arc::new(Mutex::new(false));
    }

    fn render_snakes(&mut self){
        for snake in self.snakes.iter_mut() {
            snake.go();
            snake.render(); 
        }
    }

    fn render_food(&mut self){
        // cleanup
        self.rm_expired_food();

        // new food
        let mut new_food: Vec<Food> = Vec::new();
        self.check_new_food(&mut new_food);
        if !new_food.is_empty() {
            println!("Added {} new food item(s)", new_food.len());    
        }

        self.food.append(&mut new_food);
         
        //request new food
        if self.last_food_fetched.elapsed() > FOOD_UPDATE_EVERY {
            self.request_new_food();
            self.last_food_fetched = time::Instant::now();
            println!("current number of {} food items", self.food.len());
        }

        //render
        unsafe {
            self.food.iter().for_each( move |food| {
                game_ffi::render_sprite(food.sprite);   
            });
        }
    }

    // Check whether any food has expired and remove it
    fn rm_expired_food(&mut self){
        self.food.retain(|x| x.expires.elapsed() < FOOD_EXPIRES_IN);
    }

    fn check_new_food(&self, new_food: & mut Vec<Food>) {
        let receiver = &self.channels.1;

        if !receiver.is_empty(){
            unsafe {
                receiver.try_iter().for_each(move |sprite_data| {
                    new_food.push(
                        Food{sprite: SPAWN_SPRITE!(false, sprite_data.x, sprite_data.y, 
                            sprite_data.width, sprite_data.height, sprite_data.r, sprite_data.g, sprite_data.b),
                            expires: time::Instant::now()});
                    });
            }
        }
    }

    // get new food if needed
    fn request_new_food(&self){
        println!("Request 1 more food item");
        let sender = &self.channels.0;
        let _ = sender.send(1);
    }
}