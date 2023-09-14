#![allow(dead_code)] // TODO: remove

use macroquad::prelude::*;

use smart_road::config::window_conf;
use smart_road::controls::handle_input;
use smart_road::render::render_roads;
use smart_road::traffic::*;

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
