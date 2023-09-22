use std::fmt::{Display, Formatter};
use std::time::SystemTime;

use crate::config::{MAX_VELOCITY, SCAN_AREA, SECTOR_WIDTH, WINDOW_SIZE};
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
    top: f32,
    right: f32,
    left: f32,
    bottom: f32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Car {
    pub x: f32,
    pub y: f32,
    pub moving: Moving,
    pub vel: f32,
    pub turning: Turning,
    pub path: Path,
    pub has_turned: bool,
    direction: Direction,
    time: SystemTime,
}

impl Car {
    pub fn new(direction: Direction, turning: Turning) -> Car {
        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0]);
        Car {
            x,
            y,
            moving: match &direction {
                Direction::North => Moving::Down,
                Direction::East => Moving::Left,
                Direction::South => Moving::Up,
                Direction::West => Moving::Right,
            },
            vel: 1.0,
            turning,
            path,
            has_turned: false,
            direction,
            time: SystemTime::now(),
        }
    }

    /// ### move_car
    /// Moves the car both in `Path` but also in `Car.x` and `Car.y`.
    /// Uses
    pub fn move_car(&mut self, all_cars: &[Car], grid: &Grid) {
        if !self.has_turned {
            self.update_direction();
        };

        match self.moving {
            Moving::Up => self.y -= self.vel * MAX_VELOCITY,
            Moving::Right => self.x += self.vel * MAX_VELOCITY,
            Moving::Down => self.y += self.vel * MAX_VELOCITY,
            Moving::Left => self.x -= self.vel * MAX_VELOCITY,
        }

        self.move_in_path();
        if self.turning == Turning::Right {
            self.accelerate();
            return;
        }

        if self.path.current > 7 && self.turning == Turning::Straight {
            self.accelerate();
            return;
        }

        if self.path.current > 8 && self.turning == Turning::Left {
            self.accelerate();
            return;
        }

        if self.path.current <= 2 {
            return;
        }

        // 1. Stop car if other car in same sector
        // 2. Emergency brake if car is in one sector ahead or moving towards the sector
        // 3. Brake gently if car is in two sectors ahead or car moving towards the sector
        if self.scan_sector(all_cars) {
            self.stop();
        } else if self.adjacent_sectors(grid, 1) || self.scan_in_front(all_cars) {
            self.emergency_brake();
        } else if self.adjacent_sectors(grid, 2) {
            self.brake();
        }

        // If car is in a deadlock with another car in the same sector then
        // send car who made it further into the sector.
        if !self.scan_sector(all_cars) && !self.adjacent_sectors(grid, 1) {
            self.accelerate();
        }
    }

    fn accelerate(&mut self) {
        let new_vel = (2.0 - self.vel) / 20.0;
        if self.vel < 2.0 {
            self.vel += new_vel;
        }
    }

    fn brake(&mut self) {
        self.vel *= 0.8;
    }

    fn emergency_brake(&mut self) {
        self.vel *= 0.4;
    }

    fn stop(&mut self) {
        self.vel = 0.0;
        match self.moving {
            Moving::Up => self.y += self.sector_position(),
            Moving::Right => self.x -= self.sector_position(),
            Moving::Down => self.y -= self.sector_position(),
            Moving::Left => self.x += self.sector_position(),
        }
    }

    /// ### scan_in_front
    /// Scans the sector in front of car.
    /// Uses 5 px margin to avoid scanning unwanted cars.
    fn scan_in_front(&self, cars: &[Car]) -> bool {
        let top = self.get_borders().top as usize;
        let right = self.get_borders().right as usize;
        let bottom = self.get_borders().bottom as usize;
        let left = self.get_borders().left as usize;

        for car in cars {
            if self.moving == car.moving
                || self.direction == car.direction
                || car.turning == Turning::Right
            {
                continue;
            }

            let other_top = car.get_borders().top as usize;
            let other_right = car.get_borders().right as usize;
            let other_bottom = car.get_borders().bottom as usize;
            let other_left = car.get_borders().left as usize;

            match self.moving {
                Moving::Up => {
                    for x in left + 20..right - 20 {
                        for y in (top - SCAN_AREA)..top {
                            if (other_bottom..other_top).contains(&y)
                                && (other_left..other_right).contains(&x)
                            {
                                return true;
                            }
                        }
                    }
                }
                Moving::Right => {
                    for x in right..(right + SCAN_AREA) {
                        for y in top + 20..bottom - 20 {
                            if (other_top..other_bottom).contains(&y)
                                && (other_left..other_right).contains(&x)
                            {
                                return true;
                            }
                        }
                    }
                }
                Moving::Down => {
                    for x in left + 20..right - 20 {
                        for y in bottom..(bottom + SCAN_AREA) {
                            if (other_top..other_bottom).contains(&y)
                                && (other_left..other_right).contains(&x)
                            {
                                return true;
                            }
                        }
                    }
                }
                Moving::Left => {
                    for x in (left - SCAN_AREA)..left {
                        for y in top + 15..bottom - 15 {
                            if (other_top..other_bottom).contains(&y)
                                && (other_left..other_right).contains(&x)
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// ### scan_sector
    /// Scans the current sector and checks if another car is inside the sector.
    fn scan_sector(&self, cars: &[Car]) -> bool {
        for car in cars.iter().filter(|&c| {
            let sec1 = c.get_sector();
            let sec2 = self.get_sector();
            sec1.get_x() == sec2.get_x()
                && sec1.get_y() == sec2.get_y()
                && sec1.moving != sec2.moving
        }) {
            if self.sector_position() < car.sector_position() {
                return true;
            }
        }
        false
    }

    /// ### adjacent_sectors
    /// Check for cars in chosen amount of sectors ahead. Also check adjacent sectors.
    /// This **takes into consideration** the direction the car will have in that sector.
    ///
    ///  **Calculation goes as follows:**
    /// - Up:    x-n..n, y-n
    /// - Right: x+n, y-n..n
    /// - Down:  x-n..n, y+n
    /// - Left:  x-n, y-n..n
    ///
    /// With n being `sectors_ahead`
    fn adjacent_sectors(&mut self, grid: &Grid, sectors_ahead: usize) -> bool {
        let i = self.path.current;
        let sector_ahead = &self.path.sectors[i + sectors_ahead];
        let x = sector_ahead.get_x();
        let y = sector_ahead.get_y();
        match sector_ahead.moving {
            Moving::Up | Moving::Down => {
                if let Some(car) = grid.get_car_at_coords(x - sectors_ahead, y) {
                    if car.moving == Moving::Right && car.turning != Turning::Right {
                        return self.sector_position() < car.sector_position();
                    }
                }
                if grid.get_car_at_coords(x, y).is_some() {
                    return true;
                }
                if let Some(car) = grid.get_car_at_coords(x + sectors_ahead, y) {
                    if car.moving == Moving::Left && car.turning != Turning::Right {
                        return self.sector_position() < car.sector_position();
                    }
                }
            }
            Moving::Right | Moving::Left => {
                if let Some(car) = grid.get_car_at_coords(x, y - sectors_ahead) {
                    if car.moving == Moving::Down && car.turning != Turning::Right {
                        return self.sector_position() < car.sector_position();
                    }
                }
                if grid.get_car_at_coords(x, y).is_some() {
                    return true;
                }
                if let Some(car) = grid.get_car_at_coords(x, y + sectors_ahead) {
                    if car.moving == Moving::Up && car.turning != Turning::Right {
                        return self.sector_position() < car.sector_position();
                    }
                }
            }
        }
        false
    }

    /// ### get_sector
    /// Get the current sector of a `Car`.
    pub fn get_sector(&self) -> Sector {
        self.path.sectors[self.path.current].clone()
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
    fn sector_position(&self) -> f32 {
        match self.moving {
            Moving::Up => self.get_sector().get_y() as f32 * SECTOR_WIDTH - self.y,
            Moving::Right => self.x - self.get_sector().get_x() as f32 * SECTOR_WIDTH,
            Moving::Down => self.y - self.get_sector().get_y() as f32 * SECTOR_WIDTH,
            Moving::Left => self.get_sector().get_x() as f32 * SECTOR_WIDTH - self.x,
        }
    }

    /// ### move_in_path
    /// Moves the car inside its own `Path` by incrementing `path.current`.
    fn move_in_path(&mut self) {
        let next_index = self.path.current + 1;
        if next_index >= self.path.sectors.len() {
            return;
        }

        let next = &self.path.sectors[next_index];

        match self.moving {
            Moving::Up => {
                if self.update_up(next) {
                    // println!("{}", self);
                    self.path.current += 1;
                }
            }
            Moving::Right => {
                if self.update_right(next) {
                    // println!("{}", self);
                    self.path.current += 1;
                }
            }
            Moving::Down => {
                if self.update_down(next) {
                    // println!("{}", self);
                    self.path.current += 1;
                }
            }
            Moving::Left => {
                if self.update_left(next) {
                    // println!("{}", self);
                    self.path.current += 1;
                }
            }
        }
    }

    /// ### update_direction
    /// Updates the direction of the car based on current sector in `Path`
    pub fn update_direction(&mut self) {
        if self.has_turned {
            return;
        }

        let next_index = self.path.current + 1;
        let next = &self.path.sectors[next_index];

        match self.direction {
            Direction::North => {
                if self.update_down(next) && self.update_left(next) {
                    self.moving = Moving::Right;
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_down(next) && self.update_right(next) {
                    self.moving = Moving::Left;
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
            Direction::East => {
                if self.update_left(next) && self.update_up(next) {
                    self.moving = Moving::Down;
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_left(next) && self.update_down(next) {
                    self.moving = Moving::Up;
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
            Direction::South => {
                if self.update_up(next) && self.update_left(next) {
                    self.moving = Moving::Right;
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_up(next) && self.update_right(next) {
                    self.moving = Moving::Left;
                    self.y = next.get_y() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
            Direction::West => {
                if self.update_right(next) && self.update_up(next) {
                    self.moving = Moving::Down;
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
                if self.update_right(next) && self.update_down(next) {
                    self.moving = Moving::Up;
                    self.x = next.get_x() as f32 * SECTOR_WIDTH;
                    self.has_turned = true;
                }
            }
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
            Moving::Up => self.y <= 0.0,
            Moving::Right => self.x >= WINDOW_SIZE as f32 - SECTOR_WIDTH,
            Moving::Down => self.y >= WINDOW_SIZE as f32 - SECTOR_WIDTH,
            Moving::Left => self.x <= 0.0,
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
                Sector: (x: {}, y: {}, Moving: {:?})\n\
                ------------------",
            self.x,
            self.y,
            self.vel,
            self.turning,
            self.path.current,
            self.path.sectors[self.path.current].get_x(),
            self.path.sectors[self.path.current].get_y(),
            self.path.sectors[self.path.current].moving
        )
    }
}
