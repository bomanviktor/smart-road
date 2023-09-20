use crate::config::{SCAN_AREA, SECTOR_WIDTH, WINDOW_SIZE};
use crate::traffic::path::{Path, Sector};
use crate::traffic::{Direction, Grid, Statistics};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Turning {
    Left,
    Straight,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
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

    // Add functionality here
    pub fn move_car(&mut self, grid: &Grid) {
        if !self.has_turned {
            self.update_direction();
        };
        self.move_in_path();
        if self.should_brake(grid) {
            self.vel = 0.0;
        } else {
            self.vel = 1.0;
        }

        match self.moving {
            Moving::Up => self.y -= self.vel,
            Moving::Right => self.x += self.vel,
            Moving::Down => self.y += self.vel,
            Moving::Left => self.x -= self.vel,
        }
    }

    pub fn should_brake(&mut self, grid: &Grid) -> bool {
        if self.nothing_ahead(grid) {
            return false;
        }
        for car in grid.sectors.iter().flatten() {
            if car.is_none() {
                continue;
            }

            // get scanning-borders
            let borders = self.get_borders();
            let scan_top = borders.top as usize - SCAN_AREA;
            let scan_right = borders.right as usize + SCAN_AREA;
            let scan_bottom = borders.bottom as usize + SCAN_AREA;
            let scan_left = borders.left as usize - SCAN_AREA;

            // get other car borders
            let car = car.clone().unwrap();
            let top = car.get_borders().top as usize;
            let right = car.get_borders().right as usize;
            let bottom = car.get_borders().bottom as usize;
            let left = car.get_borders().left as usize;

            match self.moving {
                Moving::Up => {
                    for x in scan_left..scan_right {
                        for y in borders.top as usize..scan_top {
                            if (top..bottom).contains(&y) && (left..right).contains(&x) {
                                return true;
                            }
                        }
                    }
                }
                Moving::Right => {
                    for x in borders.right as usize..scan_right {
                        for y in scan_top..scan_bottom {
                            if (top..bottom).contains(&y) && (left..right).contains(&x) {
                                return true;
                            }
                        }
                    }
                }
                Moving::Down => {
                    for x in scan_left..scan_right {
                        for y in borders.bottom as usize..scan_bottom {
                            if (top..bottom).contains(&y) && (left..right).contains(&x) {
                                return true;
                            }
                        }
                    }
                }
                Moving::Left => {
                    for x in borders.left as usize..scan_left {
                        for y in scan_top..scan_bottom {
                            if (top..bottom).contains(&y) && (left..right).contains(&x) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn nothing_ahead(&mut self, grid: &Grid) -> bool {
        // Always move if turning right.
        if self.turning == Turning::Right {
            return true;
        }

        // Check inside lanes
        if self.path.current < 5 {
            return self.check_next_car(grid);
        }

        // Both paths should keep going if they made it into this sector
        if self.path.current >= 7 {
            return true;
        }

        // Loop through all the cars ahead, if future collision. Stop the car.
        for (i, car) in grid.get_cars_ahead(self).into_iter().enumerate() {
            if car.is_none() {
                continue;
            }

            let car = car.unwrap();
            if car.moving == self.moving || car.direction == self.direction {
                continue;
            }
            match self.moving {
                Moving::Up => {
                    if i == 0 && car.moving == Moving::Right {
                        return false;
                    }
                    if i == 1 {
                        return false;
                    }
                    if i == 2 && car.sector_position() > SECTOR_WIDTH / 2.0 {
                        return false;
                    }
                }
                Moving::Right => {
                    if i == 0 && car.moving == Moving::Down {
                        return false;
                    }
                    if i == 1 {
                        return false;
                    }
                    if i == 2 && car.sector_position() > SECTOR_WIDTH / 2.0 {
                        return false;
                    }
                }
                Moving::Down => {
                    if i == 0 && car.moving == Moving::Right {
                        return false;
                    }
                    if i == 1 {
                        return false;
                    }
                    if i == 2 && car.sector_position() > SECTOR_WIDTH / 2.0 {
                        return false;
                    }
                }
                Moving::Left => {
                    if i == 0 && car.moving == Moving::Down {
                        return false;
                    }
                    if i == 1 {
                        return false;
                    }
                    if i == 2 && car.sector_position() > SECTOR_WIDTH / 2.0 {
                        return false;
                    }
                }
            }
            // println!("{}", car);
            //println!("---------");
        }
        false
    }

    fn check_next_car(&self, grid: &Grid) -> bool {
        let sector = self.get_sector();
        match self.moving {
            Moving::Down => {
                let other_car = grid.get_car_at_coords(sector.get_x(), sector.get_y() + 1);

                if other_car.is_none() {
                    return true;
                }

                other_car.unwrap().get_borders().top <= self.get_borders().bottom
            }

            Moving::Left => {
                let other_car = grid.get_car_at_coords(sector.get_x() - 1, sector.get_y());

                if other_car.is_none() {
                    return true;
                }

                other_car.unwrap().get_borders().right <= self.get_borders().left
            }

            Moving::Up => {
                let other_car = grid.get_car_at_coords(sector.get_x(), sector.get_y() - 1);

                if other_car.is_none() {
                    return true;
                }

                other_car.unwrap().get_borders().bottom <= self.get_borders().top
            }

            Moving::Right => {
                let other_car = grid.get_car_at_coords(sector.get_x() + 1, sector.get_y());

                if other_car.is_none() {
                    return true;
                }

                other_car.unwrap().get_borders().left <= self.get_borders().right
            }
        }
    }

    pub fn get_sector(&self) -> Sector {
        self.path.sectors[self.path.current].clone()
    }
    pub fn get_borders(&self) -> Borders {
        Borders {
            top: self.y,
            right: self.x + SECTOR_WIDTH,
            bottom: self.y + SECTOR_WIDTH,
            left: self.x,
        }
    }

    fn sector_position(&self) -> f32 {
        match self.moving {
            Moving::Up => self.y - self.get_sector().get_y() as f32 * SECTOR_WIDTH,
            Moving::Right => self.get_sector().get_x() as f32 * SECTOR_WIDTH - self.x,
            Moving::Down => self.get_sector().get_y() as f32 * SECTOR_WIDTH - self.y,
            Moving::Left => self.x - self.get_sector().get_x() as f32 * SECTOR_WIDTH - self.x,
        }
    }

    fn move_in_path(&mut self) {
        let next_index = self.path.current + 1;
        if next_index >= self.path.sectors.len() {
            return;
        }

        let next = &self.path.sectors[next_index];

        match self.moving {
            Moving::Up => {
                if self.update_up(next) {
                    self.path.current += 1;
                }
            }
            Moving::Right => {
                if self.update_right(next) {
                    self.path.current += 1;
                }
            }
            Moving::Down => {
                if self.update_down(next) {
                    self.path.current += 1;
                }
            }
            Moving::Left => {
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

    /*
    pub fn update_in_grid(&self, grid: &mut Grid) {
        let x = self.path.sectors[self.path.current].get_x();
        let y = self.path.sectors[self.path.current].get_y();
    }


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
