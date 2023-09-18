use crate::config::SECTOR_WIDTH;
use crate::traffic::path::{Path, Sector};
use crate::traffic::Direction;

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
            turning: turning.clone(),
            path,
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
    /*
        pub fn accelerate(&mut self, acceleration: f32) {
            self.vel += acceleration
        }

        pub fn de_accelerate(&mut self, de_acceleration: f32) {
            self.vel -= de_acceleration
        }
    */
}

fn get_entry_coords(p: &Sector) -> (f32, f32) {
    (
        SECTOR_WIDTH * p.get_x() as f32,
        SECTOR_WIDTH * p.get_y() as f32,
    )
}
