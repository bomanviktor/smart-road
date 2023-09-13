use crate::traffic::Direction;
use crate::traffic::state::Turning;

pub struct Path {
    direction: Direction,
    turning: Turning,
    start_x: f64,
    start_y: f64,
    breakpoint_x: f64,
    breakpoint_y: f64
}



impl Path {
    pub fn new(direction: Direction, turning: Turning) -> Path {
        Path {
            direction,
            turning,
            start_x: 0.0,
            start_y: 0.0,
            breakpoint_y: 0.0,
            breakpoint_x: 0.0
        }
    }
}

