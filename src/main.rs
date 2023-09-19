#![allow(dead_code)] // TODO: remove

use macroquad::prelude::*;

use smart_road::config::window_conf;
use smart_road::controls::handle_input;
//use smart_road::render::render_roads;
use smart_road::traffic::*;
//use smart_road::render::textures::Textures;
use smart_road::render::roads::render_textured_roads;

use smart_road::render::grid::render_grid;

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();
    let textures = smart_road::render::textures::Textures::load().await;
    loop {
        clear_background(BLACK);
        handle_input(&mut state);
        state.update();

        //render_roads();
        render_textured_roads(&textures);
        render_grid();

        next_frame().await
    }
}
