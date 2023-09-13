use crate::traffic::{Direction, State};
use macroquad::prelude::*;

pub fn handle_input(_state: &mut State) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if is_key_pressed(KeyCode::Up) {
        println!("Car from: {:?}", Direction::South);
       //traffic_state.add_car(Direction::South);
    }

    if is_key_pressed(KeyCode::Down) {
        println!("Car from: {:?}", Direction::North);
        //traffic_state.add_car(Direction::North);
    }

    if is_key_pressed(KeyCode::Right) {
        println!("Car from: {:?}", Direction::West);
        //traffic_state.add_car(Direction::West);
    }

    if is_key_pressed(KeyCode::Left) {
        println!("Car from: {:?}", Direction::East);
        //traffic_state.add_car(Direction::East);
    }

    if is_key_pressed(KeyCode::R) {
        println!("Car from: Random");
        //traffic_state.add_car_random();
    }
}