use crate::config::SECTOR_WIDTH;
use crate::traffic::path::{Path, Sector};
use crate::traffic::Direction;
use macroquad::rand::gen_range;

#[derive(Clone)]
pub enum Turning {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Debug)]
pub struct Car {
    x: f32,
    y: f32,
    vel: f32,
    direction: Direction,
    pub path: Path,
}

impl Car {
    pub fn new(direction: Direction) -> Car {
        let turning = match gen_range(0, 2) {
            0 => Turning::Left,
            1 => Turning::Straight,
            _ => Turning::Right,
        };

        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0]);
        let car = Car {
            x,
            y,
            vel: 1.0,
            direction,
            path,
        };
        println!("{:?}", car.path);
        car
    }

    // Add functionality here
    pub fn move_car(&mut self) {
        match self.direction {
            Direction::North => {}
            Direction::East => {}
            Direction::South => {}
            Direction::West => {}
        }
    }

    pub fn accelerate(&mut self, acceleration: f32) {
        self.vel += acceleration
    }

    pub fn de_accelerate(&mut self, de_acceleration: f32) {
        self.vel -= de_acceleration
    }
}

fn get_entry_coords(p: &Sector) -> (f32, f32) {
    (
        SECTOR_WIDTH * p.get_x() as f32,
        SECTOR_WIDTH * p.get_y() as f32,
    )
}
