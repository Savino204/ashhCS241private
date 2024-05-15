mod config;
mod pong;

use bevy::prelude::*;
use config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pong::PongGame;
use rand::Rng; // 0.8.5
use std::fs;

fn main() {
    //player_random_number_gen();
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusty Pong".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            PongGame,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

pub fn player_random_number_gen(){
    let num: i32 = rand::thread_rng().gen_range(0..100);
    let contents = num.to_string();
    fs::write("player_num.txt", contents).unwrap();
}
