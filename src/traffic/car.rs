use macroquad::rand::{gen_range, rand};
use crate::traffic::Direction;
use crate::traffic::state::Turning;
use crate::traffic::path::Path;

#[allow(dead_code)]
pub struct Car {
    x: f64,
    y: f64,
    vel: f64,
    path: Path
}
#[allow(dead_code)]
impl Car {
    pub fn new(direction: Direction) -> Car {
        let turning = match gen_range(0, 2) {
            0 => Turning::Left,
            1 => Turning::Straight,
            _ => Turning::Right,
        };
        Car {
            x: 0.0,
            y: 0.0,
            vel: 0.0,
            path: Path::new(direction, turning)
        }
    }
}