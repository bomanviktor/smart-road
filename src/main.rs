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
    let mut state = State::new();
    loop {
        clear_background(BLACK);
        handle_input(&mut state);

        render_roads();

        next_frame().await
    }
}
