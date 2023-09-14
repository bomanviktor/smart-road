use crate::traffic::{Direction, State};
use macroquad::prelude::*;

pub fn handle_input(state: &mut State) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if is_key_pressed(KeyCode::Up) {
        state.add_car(Direction::South);
    }

    if is_key_pressed(KeyCode::Down) {
        state.add_car(Direction::North);
    }

    if is_key_pressed(KeyCode::Right) {
        state.add_car(Direction::West);
    }

    if is_key_pressed(KeyCode::Left) {
        state.add_car(Direction::East);
    }

    if is_key_pressed(KeyCode::R) {
        state.add_car_random();
    }
}
