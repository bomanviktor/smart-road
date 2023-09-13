use crate::traffic::car::Car;
use crate::traffic::intersection::Intersection;
use crate::traffic::lane::Lane;
use crate::traffic::statistics::*;

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}


pub struct State {
    lanes: [Lane; 4],
    intersection: Intersection,
    stats: Statistics
}

impl State {
    pub fn new() -> State {
        State {
            lanes: [
                Lane::new(Direction::North),
                Lane::new(Direction::East),
                Lane::new(Direction::South),
                Lane::new(Direction::West)],
            intersection: Intersection::new(),
            stats: Statistics::new()
        }
    }
}



