use crate::config::ROAD_WIDTH;
use crate::traffic::path::{Path, Sector};
use crate::traffic::Direction;
use macroquad::rand::gen_range;

#[derive(Clone)]
pub enum Moving {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
pub enum Turning {
    Left,
    Straight,
    Right,
}

#[derive(Clone)]
pub struct Car {
    x: f32,
    y: f32,
    vel: f32,
    direction: Direction,
    turning: Turning,
    moving: Moving,
    pub path: Path,
}
#[allow(dead_code)]
impl Car {
    pub fn new(direction: Direction) -> Car {
        let turning = match gen_range(0, 2) {
            0 => Turning::Left,
            1 => Turning::Straight,
            _ => Turning::Right,
        };

        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0]);
        Car {
            x,
            y,
            vel: 1.0,
            moving: moving(&direction),
            direction,
            turning,
            path,
        }
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

fn moving(direction: &Direction) -> Moving {
    match direction {
        Direction::North => Moving::Down,
        Direction::East => Moving::Left,
        Direction::South => Moving::Up,
        Direction::West => Moving::Right,
    }
}

fn get_entry_coords(p: &Sector) -> (f32, f32) {
    (
        ROAD_WIDTH / 2.0 + (80.0 * p.get_x() as f32),
        ROAD_WIDTH / 2.0 + (80.0 * p.get_y() as f32),
    )
}
