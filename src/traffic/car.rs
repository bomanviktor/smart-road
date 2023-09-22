use std::fmt::{Display, Formatter};
use std::time::SystemTime;

use rand::prelude::IteratorRandom;

use crate::config::{SECTOR_WIDTH, WINDOW_SIZE};
use crate::traffic::path::{Path, Sector};
use crate::traffic::{Direction, Grid, Statistics};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    Standard,
    Audi,
    Truck,
    Van,
    Taxi,
    Viper,
}
// Adjust odds of getting certain cars here. Might scratch and use `gen_range` instead
const MODELS: [Model; 7] = [
    Model::Standard,
    Model::Standard,
    Model::Audi,
    Model::Truck,
    Model::Van,
    Model::Taxi,
    Model::Viper,
];

#[derive(PartialEq, Clone, Debug)]
pub struct Car {
    pub x: f32,
    pub y: f32,
    pub vel: Velocity,
    pub turning: Turning,
    pub path: Path,
    pub has_turned: bool,
    direction: Direction,
    time: SystemTime,
    pub model: Model,
}

impl Car {
    pub fn new(direction: Direction, turning: Turning) -> Car {
        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0]);
        Car {
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
            has_turned: false,
            direction,
            time: SystemTime::now(),
            model: MODELS.into_iter().choose(&mut rand::thread_rng()).unwrap(),
        }
    }

    // Add functionality here
    pub fn move_car(&mut self) {
        if !self.has_turned {
            self.update_direction();
        };
        self.move_in_path();
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

    fn move_in_path(&mut self) {
        let next_index = self.path.current + 1;
        if next_index >= self.path.sectors.len() {
            return;
        }

        let next = &self.path.sectors[next_index];

        match self.vel {
            Velocity::Up(_) => {
                if self.update_up(next) {
                    self.path.current += 1;
                }
            }
            Velocity::Right(_) => {
                if self.update_right(next) {
                    self.path.current += 1;
                }
            }
            Velocity::Down(_) => {
                if self.update_down(next) {
                    self.path.current += 1;
                }
            }
            Velocity::Left(_) => {
                if self.update_left(next) {
                    self.path.current += 1;
                }
            }
        }
    }

    fn update_up(&self, next: &Sector) -> bool {
        self.y < next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_right(&self, next: &Sector) -> bool {
        self.x > next.get_x() as f32 * SECTOR_WIDTH
    }

    fn update_down(&self, next: &Sector) -> bool {
        self.y > next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_left(&self, next: &Sector) -> bool {
        self.x < next.get_x() as f32 * SECTOR_WIDTH
    }

    pub fn update_direction(&mut self) {
        let next_index = self.path.current + 1;
        if next_index > self.path.sectors.len() / 2 + 1 {
            return;
        }

        let velocity = self.get_velocity();
        let next = &self.path.sectors[next_index];

        match self.direction {
            Direction::North => {
                if self.update_down(next) && self.update_left(next) {
                    self.vel = Velocity::Right(velocity);
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_down(next) && self.update_right(next) {
                    self.vel = Velocity::Left(velocity);
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
            Direction::East => {
                if self.update_left(next) && self.update_up(next) {
                    self.vel = Velocity::Down(velocity);
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_left(next) && self.update_down(next) {
                    self.vel = Velocity::Up(velocity);
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
            Direction::South => {
                if self.update_up(next) && self.update_left(next) {
                    self.vel = Velocity::Right(velocity);
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_up(next) && self.update_right(next) {
                    self.vel = Velocity::Left(velocity);
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
            Direction::West => {
                if self.update_right(next) && self.update_up(next) {
                    self.vel = Velocity::Down(velocity);
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_right(next) && self.update_down(next) {
                    self.vel = Velocity::Up(velocity);
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
        }
    }

    pub fn update_in_grid(&self, grid: &mut Grid) {
        let x = self.path.sectors[self.path.current].get_x();
        let y = self.path.sectors[self.path.current].get_y();

        if (3..=8).contains(&x) && (3..=8).contains(&y) {
            grid.insert_car_to_intersection(self.clone());
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
        match self.vel {
            Velocity::Up(_) => self.y <= 0.0,
            Velocity::Right(_) => self.x >= WINDOW_SIZE as f32 - SECTOR_WIDTH,
            Velocity::Down(_) => self.y >= WINDOW_SIZE as f32 - SECTOR_WIDTH,
            Velocity::Left(_) => self.x <= 0.0,
        }
    }
}

fn get_entry_coords(p: &Sector) -> (f32, f32) {
    (
        SECTOR_WIDTH * p.get_x() as f32,
        SECTOR_WIDTH * p.get_y() as f32,
    )
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x: {}, y: {})\n\
                Velocity: {:?}\n\
                Turning: {:?}\n\
                Sector index: {}\n\
                Current sector: (x: {}, y: {})",
            self.x,
            self.y,
            self.vel,
            self.turning,
            self.path.current,
            self.path.sectors[self.path.current].get_x(),
            self.path.sectors[self.path.current].get_y()
        )
    }
}
