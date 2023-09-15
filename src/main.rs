#![allow(dead_code)] // TODO: remove

use macroquad::prelude::*;

use smart_road::config::window_conf;
use smart_road::controls::handle_input;
use smart_road::render::render_roads;
use smart_road::traffic::*;

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
