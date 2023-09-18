use crate::config::SECTOR_WIDTH;
use crate::traffic::path::{Path, Sector};
use crate::traffic::Direction;
use macroquad::rand::gen_range;

#[derive(Eq, PartialEq, Clone)]
pub enum Turning {
    Left,
    Straight,
    Right,
}

pub enum Velocity {
    Up(f32),
    Right(f32),
    Down(f32),
    Left(f32)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Car {
    x: f32,
    y: f32,
    vel: Velocity,
    direction: Direction,
    pub path: Path,
}

impl Car {
    pub fn new(direction: Direction) -> Car {
        let turning = match gen_range(0, 3) {
            0 => Turning::Left,
            1 => Turning::Straight,
            _ => Turning::Right,
        };

        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0]);
        let car = Car {
            x,
            y,
            vel: match &direction {
                Direction::North => Velocity::Down(1.0),
                Direction::East => Velocity::Left(1.0),
                Direction::South => Velocity::Up(1.0),
                Direction::West => Velocity::Right(1.0),
            },
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
