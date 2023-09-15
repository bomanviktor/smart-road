#![allow(dead_code)] // TODO: remove
mod config;
mod controls;
mod render;
mod traffic;

use macroquad::prelude::*;

use crate::controls::handle_input;
use crate::render::render_roads;
use crate::traffic::State;
use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    pub const WINDOW_SIZE: i32 = 1000;
    let mut state = State::new();
    loop {
        macroquad::window::request_new_screen_size(WINDOW_SIZE as f32, WINDOW_SIZE as f32);
        clear_background(BLACK);
        handle_input(&mut state);

        render_roads();

        next_frame().await
    }
}
