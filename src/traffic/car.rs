use crate::config::{SECTOR_WIDTH, WINDOW_SIZE};
use crate::traffic::path::{Path, Sector};
use crate::traffic::{Direction, Statistics};
use std::time::SystemTime;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Turning {
    Left,
    Straight,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Velocity {
    Up(f32),
    Right(f32),
    Down(f32),
    Left(f32),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Car {
    x: f32,
    y: f32,
    vel: Velocity,
    pub turning: Turning,
    pub path: Path,
    time: SystemTime,
}

impl Car {
    pub fn new(direction: Direction, turning: Turning) -> Car {
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
            turning,
            path,
            time: SystemTime::now(),
        };
        println!("{:?}", car.path);
        car
    }

    // Add functionality here
    pub fn move_car(&mut self) {
        match self.vel {
            Velocity::Up(v) => self.y -= v,
            Velocity::Right(v) => self.x += v,
            Velocity::Down(v) => self.y += v,
            Velocity::Left(v) => self.x -= v,
        }
    }

    pub fn get_velocity(&self) -> f32 {
        match self.vel {
            Velocity::Up(value) => value,
            Velocity::Right(value) => value,
            Velocity::Down(value) => value,
            Velocity::Left(value) => value,
        }
    }
    /*
        pub fn accelerate(&mut self, acceleration: f32) {
            self.vel += acceleration
        }

        pub fn de_accelerate(&mut self, de_acceleration: f32) {
            self.vel -= de_acceleration
        }
    */

    pub fn add_time(&self, stats: &mut Statistics) {
        let duration = SystemTime::now().duration_since(self.time).unwrap();
        stats.set_time(duration.as_secs_f32());
    }

    // Check if the car has reached its destination
    pub fn is_done(&self) -> bool {
        if self.path.sectors.len() - 1 != self.path.current {
            return false;
        }

        let last_sector = self.path.sectors.iter().next_back().unwrap();

        if last_sector.get_x() == 0 {
            return self.x == 0.0;
        }

        if last_sector.get_x() == 11 {
            return self.x == WINDOW_SIZE as f32 - SECTOR_WIDTH;
        }

        if last_sector.get_y() == 0 {
            return self.y == 0.0;
        }

        if last_sector.get_y() == 11 {
            return self.y == WINDOW_SIZE as f32 - SECTOR_WIDTH;
        }

        false
    }
}

fn get_entry_coords(p: &Sector) -> (f32, f32) {
    (
        SECTOR_WIDTH * p.get_x() as f32,
        SECTOR_WIDTH * p.get_y() as f32,
    )
}
