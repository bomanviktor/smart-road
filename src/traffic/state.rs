use crate::traffic::car::Car;
use crate::traffic::lane::Lane;
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
    intersection: [[Option<Car>; 6];6], // Matrix displaying all available spots in intersection
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
            intersection: [[None;6];6],
            stats: Statistics::new()
        }
    }
}



