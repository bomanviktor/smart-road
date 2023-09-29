use std::fmt::{Display, Formatter};
use std::time::SystemTime;

use rand::prelude::IteratorRandom;

use crate::traffic::path::{Path, Sector};
use crate::traffic::{Direction, Statistics};

use crate::config::{
    ACCELERATION_DISTANCE, MAX_VELOCITY, SCAN_AREA, SECTOR_WIDTH, SPEED_LIMIT, WINDOW_SIZE,
};

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
    pub model: Model,
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
            model: MODELS.into_iter().choose(&mut rand::thread_rng()).unwrap(),
        }
    }

    /// ### move_car
    /// Moves the car both in `Path` but also in `Car.x` and `Car.y`.
    pub fn move_car(&mut self, all_cars: &[Car]) {
        self.move_in_path(all_cars);
        self.moving = self.sector(0).moving;
        self.change_pos(all_cars);

        // car is turning right, no further logic needed
        if self.turning == Turning::Right {
            self.accelerate(SCAN_AREA);
            return;
        }

        // car is still entering the intersection
        if self.index < 2 {
            return;
        }

        // car going straight has reached the other side of the intersection
        if self.index >= 7 {
            self.forward_scan(all_cars);
            return;
        }

        if self.turning == Turning::Left && (5..=7).contains(&self.index) {
            self.center_scan(all_cars);
        }

        // Adjust position in x and y axis
        self.adjust_position();

        // send rays a certain distance and check for cars
        self.ray_casting(all_cars);

        // scan in front of car to see if it is safe to accelerate, or if it should stop
        self.forward_scan(all_cars);
    }

    pub fn accelerate(&mut self, distance: f32) {
        // TODO: improve in `acceleration`
        let x = if distance > ACCELERATION_DISTANCE {
            1.0
        } else {
            distance / ACCELERATION_DISTANCE
        };
        let new_vel = ((SPEED_LIMIT - self.vel) / 60.0) * x;
        if self.vel < SPEED_LIMIT {
            self.vel += new_vel;
        }
    }

    pub fn brake(&mut self, distance: f32) {
        let new_vel = self.vel - distance / SCAN_AREA;
        if new_vel < 0.0 {
            return;
        }

        self.vel -= new_vel;
        if self.vel < 0.3 {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.vel = 0.0;
    }

    fn change_pos(&mut self, cars: &[Car]) {
        let x = match cars
            .iter()
            .filter(|c| self.calc_dist(c) < ACCELERATION_DISTANCE)
            .count()
        {
            0 => 1.1,
            1 => 1.0,
            _ => 0.9,
        };
        match self.moving {
            Moving::Up => self.y -= self.vel * MAX_VELOCITY * x,
            Moving::Right => self.x += self.vel * MAX_VELOCITY * x,
            Moving::Down => self.y += self.vel * MAX_VELOCITY * x,
            Moving::Left => self.x -= self.vel * MAX_VELOCITY * x,
        }
    }

    /// ### sector_position
    /// Get the distance travelled into a `Sector`. This is used to break deadlocks.
    pub fn sector_pos(&self) -> f32 {
        match self.moving {
            Moving::Up => SECTOR_WIDTH - (self.y - self.sector(0).get_y() as f32 * SECTOR_WIDTH),
            Moving::Right => SECTOR_WIDTH - (self.sector(0).get_x() as f32 * SECTOR_WIDTH - self.x),
            Moving::Down => SECTOR_WIDTH - (self.sector(0).get_y() as f32 * SECTOR_WIDTH - self.y),
            Moving::Left => SECTOR_WIDTH - (self.x - self.sector(0).get_x() as f32 * SECTOR_WIDTH),
        }
    }

    /// ### move_in_path
    /// Moves the car inside its own `Path` by incrementing `path.current`.
    /// Stop if a car in sector ahead.
    fn move_in_path(&mut self, cars: &[Car]) {
        if self.index + 2 > self.path.sectors.len() {
            return;
        }
        let car_ahead = cars.iter().any(|c| c.sector(0) == self.sector(1));

        let next = &self.sector(0);
        match self.moving {
            Moving::Up => {
                if self.update_up(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
            Moving::Right => {
                if self.update_right(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
            Moving::Down => {
                if self.update_down(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
            Moving::Left => {
                if self.update_left(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
        }
    }

    // Helper functions for `move_in_path`
    fn update_up(&self, next: &Sector) -> bool {
        self.y - self.vel * MAX_VELOCITY <= next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_right(&self, next: &Sector) -> bool {
        self.x + self.vel * MAX_VELOCITY >= next.get_x() as f32 * SECTOR_WIDTH
    }

    fn update_down(&self, next: &Sector) -> bool {
        self.y + self.vel * MAX_VELOCITY >= next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_left(&self, next: &Sector) -> bool {
        self.x - self.vel * MAX_VELOCITY <= next.get_x() as f32 * SECTOR_WIDTH
    }

    /// ### update_direction
    /// Updates the direction of the car based on current sector in `Path`
    pub fn adjust_position(&mut self) {
        let previous = self.index - 1;
        let previous_sector = &self.path.sectors[previous];

        let sector = self.sector(0);

        if sector.get_x() != previous_sector.get_x() {
            self.y = SECTOR_WIDTH * sector.get_y() as f32;
        }

        if sector.get_y() != previous_sector.get_y() {
            self.x = SECTOR_WIDTH * sector.get_x() as f32;
        }
    }

    /// ### get_sector
    /// Get the sector of a `Car` specified by `n`.
    pub fn sector(&self, n: isize) -> Sector {
        let i = if n < 0 {
            self.index - (-n as usize)
        } else {
            self.index + n as usize
        };

        self.path.sectors[i].clone()
    }

    /// ### get_borders
    /// Get the borders of a `Car`.
    pub fn borders(&self) -> Borders {
        Borders {
            top: self.y,
            right: self.x + SECTOR_WIDTH,
            bottom: self.y + SECTOR_WIDTH,
            left: self.x,
        }
    }

    pub fn add_time(&self, stats: &mut Statistics) {
        let duration = SystemTime::now().duration_since(self.time).unwrap();
        stats.set_time(duration.as_secs_f32());
    }

    /// ### is_done
    /// Checks if car has reached the end of their `Path`
    pub fn is_done(&self) -> bool {
        match self.moving {
            Moving::Up => self.borders().top <= 0.0,
            Moving::Right => self.borders().right >= WINDOW_SIZE as f32,
            Moving::Down => self.borders().bottom >= WINDOW_SIZE as f32,
            Moving::Left => self.borders().left <= 0.0,
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
            self.sector(0),
        )
    }
}
