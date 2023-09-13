use crate::traffic::car::Car;
use crate::traffic::lane::Lane;
use crate::traffic::state::Direction::{East, North, South, West};
use crate::traffic::statistics::*;

pub enum Turning {
    Straight,
    Left,
    Right
}
#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}


pub struct State {
    lanes: [Lane; 4],
    intersection: Vec<Car>,
    stats: Statistics
}

impl State {
    pub fn new() -> State {
        State {
            lanes: [
                Lane::new(North),
                Lane::new(East),
                Lane::new(South),
                Lane::new(West)],
            intersection: Vec::new(),
            stats: Statistics::new()
        }
    }
}



