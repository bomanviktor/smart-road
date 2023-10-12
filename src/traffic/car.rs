use macroquad::rand::gen_range;
use std::time::SystemTime;

use crate::traffic::path::{Path, Sector};
use crate::traffic::{Direction, Statistics};

use crate::config::{
    ACCELERATION_DISTANCE, CLOSE_CALL_DISTANCE, FPS, MAX_VELOCITY, SCAN_DISTANCE, SECTOR_WIDTH,
    SPEED_LIMIT, WINDOW_SIZE,
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
    Viper,
}
// Adjust odds of getting certain cars here. Might scratch and use `gen_range` instead

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
        let (x, y) = get_entry_coords(&path.sectors[0], &direction);
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
            model: match gen_range(0, 5) {
                0 => Model::Viper,
                1 => Model::Audi,
                _ => Model::Standard,
            },
        }
    }

    /// ### move_car
    /// Move the car in `Path` and also in `Car.x` and `Car.y`.
    pub fn move_car(&mut self, all_cars: &[Car]) {
        self.move_in_path(all_cars);
        self.moving = self.sector(0).moving;
        self.change_pos(all_cars);

        // car is turning right, no further logic needed
        if self.turning == Turning::Right {
            self.accelerate(SCAN_DISTANCE);
            return;
        }

        // car is still entering the intersection
        if self.index < 2 {
            return;
        }

        if self.turning == Turning::Straight && (3..=7).contains(&self.index) {
            self.sector_in_front(all_cars);
        }

        if self.index == 3 && self.sector_pos() > CLOSE_CALL_DISTANCE {
            self.check_passing(all_cars);
        }

        if self.turning == Turning::Left && (5..=7).contains(&self.index) {
            self.center_scan(all_cars);
        }

        // car going straight has reached the other side of the intersection
        if self.index >= 8 {
            self.forward_scan(all_cars);
            return;
        }

        // Adjust position in x and y axis
        self.adjust_position();

        // send rays a certain distance and check for cars
        self.ray_casting(all_cars);

        // scan in front of car to see if it is safe to accelerate, or if it should stop
        self.forward_scan(all_cars);
    }

    pub fn accelerate(&mut self, distance: f32) {
        let x = if distance >= SCAN_DISTANCE {
            1.0
        } else {
            distance / SCAN_DISTANCE
        };
        let new_vel = (SPEED_LIMIT - self.vel) / FPS as f32 * x;
        if self.vel < SPEED_LIMIT {
            self.vel += new_vel;
        }
    }

    pub fn brake(&mut self, distance: f32) {
        let new_vel = self.vel - distance / SCAN_DISTANCE;
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

    /// ### change_pos
    /// Change position of car. Will go faster if no cars around and slower if too many cars around.
    fn change_pos(&mut self, cars: &[Car]) {
        let x = match cars
            .iter()
            .filter(|c| self.id != c.id && self.calc_dist(c) < ACCELERATION_DISTANCE)
            .count()
        {
            0 => 1.05,
            1 => 1.00,
            _ => 0.90,
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
    pub fn sector(&self, n: usize) -> Sector {
        self.path.sectors[self.index + n].clone()
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
            Moving::Up => self.borders().bottom <= 0.0,
            Moving::Right => self.borders().left >= WINDOW_SIZE as f32,
            Moving::Down => self.borders().top >= WINDOW_SIZE as f32,
            Moving::Left => self.borders().right <= 0.0,
        }
    }
}

fn get_entry_coords(p: &Sector, direction: &Direction) -> (f32, f32) {
    match direction {
        Direction::West => (
            SECTOR_WIDTH * p.get_x() as f32 - SECTOR_WIDTH,
            SECTOR_WIDTH * p.get_y() as f32,
        ),
        Direction::East => (
            SECTOR_WIDTH * p.get_x() as f32 + SECTOR_WIDTH,
            SECTOR_WIDTH * p.get_y() as f32,
        ),
        Direction::North => (
            SECTOR_WIDTH * p.get_x() as f32,
            SECTOR_WIDTH * p.get_y() as f32 - SECTOR_WIDTH,
        ),
        Direction::South => (
            SECTOR_WIDTH * p.get_x() as f32,
            SECTOR_WIDTH * p.get_y() as f32 + SECTOR_WIDTH,
        ),
    }
}
