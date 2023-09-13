mod config;
mod controls;
mod traffic;
mod render;

use macroquad::prelude::*;

use config::window_conf;
use crate::controls::handle_input;
use crate::render::render_roads;
use crate::traffic::State;

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