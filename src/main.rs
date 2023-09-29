use std::thread;
use std::time::{Duration, Instant};

use macroquad::prelude::*;

use smart_road::config::{window_conf, FPS, RANDOM_INTERVAL};
use smart_road::controls::handle_input;
use smart_road::render::car::render_car;
use smart_road::render::roads::render_textured_roads;
use smart_road::render::statistics::render_statistics;
use smart_road::traffic::*;

#[macroquad::main(window_conf)]
async fn main() {
    let textures = smart_road::render::textures::Textures::load().await;
    let mut state = State::new();

    let frame_duration = Duration::from_micros(1_000_000 / FPS);
    let mut last_frame_time = Instant::now();

    let mut random_timer = Instant::now();
    let random_interval = Duration::from_millis(RANDOM_INTERVAL);

    loop {
        clear_background(BLACK);
        handle_input(&mut state);
        if !state.show_final_statistics {
            render_textured_roads(&textures);

            if state.random && random_timer.elapsed() > random_interval {
                state.add_car_random();
                random_timer = Instant::now();
            }
            state.update();

            for road in &state.roads {
                for car in road.cars.iter().flatten() {
                    render_car(car, &textures.cars);
                }
            }
            let elapsed = last_frame_time.elapsed();
            if elapsed < frame_duration {
                thread::sleep(frame_duration - elapsed);
            }

            last_frame_time = Instant::now();
        } else {
            state.stats.set_max_vehicles(state.total_cars);
            render_statistics(&state.stats);
        }
        next_frame().await
    }
}
