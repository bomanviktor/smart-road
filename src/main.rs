#![allow(dead_code)] // TODO: remove

use macroquad::prelude::*;

use smart_road::config::window_conf;
use smart_road::controls::handle_input;

use smart_road::traffic::*;

use smart_road::render::roads::render_textured_roads;

use smart_road::render::car::render_car;
use smart_road::render::grid::render_grid;

#[macroquad::main(window_conf)]
async fn main() {
    let textures = smart_road::render::textures::Textures::load().await;
    let mut state = State::new();
    loop {
        clear_background(BLACK);
        handle_input(&mut state);
        state.update();

        render_textured_roads(&textures);
        render_grid();

        state.update();

        for road in &state.roads {
            for cars in &road.cars {
                for car in cars {
                    render_car(car, &textures.car);
                }
            }
        }

        next_frame().await
    }
}
