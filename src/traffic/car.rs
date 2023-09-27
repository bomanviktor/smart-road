use std::fmt::{Display, Formatter};
use std::time::SystemTime;

use crate::config::{MAX_VELOCITY, SECTOR_WIDTH, WINDOW_SIZE};
use crate::traffic::path::{Path, Sector};
use crate::traffic::{Direction, Grid, Statistics};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Turning {
    Left,
    Straight,
    Right,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Moving {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Borders {
    pub(crate) top: f32,
    pub(crate) right: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
}

#[derive(Clone, Debug)]
pub struct Car {
    pub x: f32,
    pub y: f32,
    pub index: usize,
    pub moving: Moving,
    pub vel: f32,
    pub turning: Turning,
    pub path: Path,
    pub direction: Direction,
    pub id: usize,
    time: SystemTime,
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Car {
    pub fn new(direction: Direction, turning: Turning, id: usize) -> Car {
        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0]);
        Car {
            x,
            y,
            index: 0,
            moving: match &direction {
                Direction::North => Moving::Down,
                Direction::East => Moving::Left,
                Direction::South => Moving::Up,
                Direction::West => Moving::Right,
            },
            vel: 1.0,
            id,
            turning,
            path,
            direction,
            time: SystemTime::now(),
        }
    }

    /// ### move_car
    /// Moves the car both in `Path` but also in `Car.x` and `Car.y`.
    pub fn move_car(&mut self, all_cars: &[Car], grid: &Grid) {
        self.move_in_path();
        self.moving = self.get_sector().moving;
        match self.moving {
            Moving::Up => self.y -= self.vel * MAX_VELOCITY,
            Moving::Right => self.x += self.vel * MAX_VELOCITY,
            Moving::Down => self.y += self.vel * MAX_VELOCITY,
            Moving::Left => self.x -= self.vel * MAX_VELOCITY,
        }

        if self.turning == Turning::Right {
            self.accelerate();
            return;
        }

        if self.turning == Turning::Straight && self.index >= 7 {
            self.accelerate();
            return;
        }

        if self.turning == Turning::Left && self.index >= 9 {
            self.accelerate();
            return;
        }

        if self.index < 2 {
            return;
        }

        self.adjust_position();

        // 1. Stop car if other car in same sector
        // 2. Emergency brake if car is in one sector ahead or moving towards the sector
        // 3. Brake gently if car is in two sectors ahead or car moving towards the sector
        if let Some(car) = self.adjacent_sectors(grid, 1) {
            self.brake(self.calculate_distance(&car));
        } else if let Some(car) = self.adjacent_sectors(grid, 2) {
            self.brake(self.calculate_distance(&car));
        } else {
            self.accelerate();
        }

        self.sector_ahead(all_cars);

        if self.scan_in_front(all_cars) {
            self.stop();
        }

        if self.vel <= 0.3 {
            self.break_deadlock(all_cars);
        }
    }

    pub(crate) fn accelerate(&mut self) {
        let new_vel = (2.0 - self.vel) / 20.0;
        if self.vel < 2.0 {
            self.vel += new_vel;
        }
    }

    pub fn slow_down(&mut self) {
        self.vel *= 0.9;
    }

    pub fn brake(&mut self, distance: f32) {
        let new_vel = (distance / SECTOR_WIDTH) - 0.3;
        if new_vel > self.vel {
            return;
        }
        self.vel = new_vel;
    }

    fn stop(&mut self) {
        self.vel = 0.0;
    }

    /// ### get_sector
    /// Get the current sector of a `Car`.
    pub fn get_sector(&self) -> Sector {
        self.path.sectors[self.index].clone()
    }

    /// ### get_borders
    /// Get the borders of a `Car`.
    pub fn get_borders(&self) -> Borders {
        Borders {
            top: self.y,
            right: self.x + SECTOR_WIDTH,
            bottom: self.y + SECTOR_WIDTH,
            left: self.x,
        }
    }

    /// ### sector_position
    /// Get the distance travelled into a `Sector`. This is used to break deadlocks.
    pub(crate) fn sector_position(&self) -> f32 {
        match self.moving {
            Moving::Up => SECTOR_WIDTH - (self.y - self.get_sector().get_y() as f32 * SECTOR_WIDTH),
            Moving::Right => {
                SECTOR_WIDTH - (self.get_sector().get_x() as f32 * SECTOR_WIDTH - self.x)
            }
            Moving::Down => {
                SECTOR_WIDTH - (self.get_sector().get_y() as f32 * SECTOR_WIDTH - self.y)
            }
            Moving::Left => {
                SECTOR_WIDTH - (self.x - self.get_sector().get_x() as f32 * SECTOR_WIDTH)
            }
        }
    }

    /// ### move_in_path
    /// Moves the car inside its own `Path` by incrementing `path.current`.
    ///
    fn move_in_path(&mut self) {
        if self.index + 1 > self.path.sectors.len() {
            return;
        }
        let next = &self.get_sector();
        match self.moving {
            Moving::Up => {
                if self.update_up(next) {
                    // println!("{}", self);
                    self.index += 1;
                }
            }
            Moving::Right => {
                if self.update_right(next) {
                    // println!("{}", self);
                    self.index += 1;
                }
            }
            Moving::Down => {
                if self.update_down(next) {
                    // println!("{}", self);
                    self.index += 1;
                }
            }
            Moving::Left => {
                if self.update_left(next) {
                    // println!("{}", self);
                    self.index += 1;
                }
            }
        }
    }

    /// ### update_direction
    /// Updates the direction of the car based on current sector in `Path`
    pub fn adjust_position(&mut self) {
        let previous = self.index - 1;
        let previous_sector = &self.path.sectors[previous];

        let sector = self.get_sector();

        if sector.get_x() != previous_sector.get_x() {
            self.y = SECTOR_WIDTH * sector.get_y() as f32;
        }

        if sector.get_y() != previous_sector.get_y() {
            self.x = SECTOR_WIDTH * sector.get_x() as f32;
        }
    }

    // Helper functions for `move_in_path` and `update_direction`
    fn update_up(&self, next: &Sector) -> bool {
        self.y <= next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_right(&self, next: &Sector) -> bool {
        self.x >= next.get_x() as f32 * SECTOR_WIDTH
    }

    fn update_down(&self, next: &Sector) -> bool {
        self.y >= next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_left(&self, next: &Sector) -> bool {
        self.x <= next.get_x() as f32 * SECTOR_WIDTH
    }

    pub fn add_time(&self, stats: &mut Statistics) {
        let duration = SystemTime::now().duration_since(self.time).unwrap();
        stats.set_time(duration.as_secs_f32());
    }

    /// ### is_done
    /// Checks if car has reached the end of their `Path`
    pub fn is_done(&self) -> bool {
        match self.moving {
            Moving::Up => self.get_borders().top <= 0.0,
            Moving::Right => self.get_borders().right >= WINDOW_SIZE as f32,
            Moving::Down => self.get_borders().bottom >= WINDOW_SIZE as f32,
            Moving::Left => self.get_borders().left <= 0.0,
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
                Id: {} \n\
                Velocity: {:?}\n\
                Turning: {:?}\n\
                Moving: {:?}\n\
                Sector index: {}\n\
                {}\n\
                ------------------",
            self.x,
            self.y,
            self.id,
            self.vel,
            self.turning,
            self.moving,
            self.index,
            self.get_sector(),
        )
    }
}
