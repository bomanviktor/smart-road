use crate::config::{SCAN_AREA, SECTOR_WIDTH, WINDOW_SIZE};
use crate::traffic::*;

impl Car {
    /// ### forward_scan
    /// Scans the sectors in front of the car
    ///
    pub fn forward_scan(&mut self, cars: &[Car]) {
        // Get the ranges where we scan cars in front
        let scan_x = self.get_borders().left..=self.get_borders().right;
        let scan_y = self.get_borders().top..=self.get_borders().bottom;

        // The longest distance to car in front.
        let mut distance = WINDOW_SIZE as f32;
        for car in cars.iter().filter(|c| c.id != self.id) {
            if self.calculate_distance(car) > distance {
                continue;
            }

            let (x, y) = self.center_car();
            let (x2, y2) = car.center_car();
            match self.moving {
                Moving::Up => {
                    if y < y2 {
                        continue;
                    }
                    if scan_x.contains(&x2) {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Down => {
                    if y > y2 {
                        continue;
                    }
                    if scan_x.contains(&x2) {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Right => {
                    if x > x2 {
                        continue;
                    }
                    if scan_y.contains(&y2) {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Left => {
                    if x < x2 {
                        continue;
                    }
                    if scan_y.contains(&y2) {
                        distance = self.calculate_distance(car);
                    }
                }
            }
        }
        if distance > SECTOR_WIDTH {
            self.accelerate(distance);
        } else {
            self.stop();
        }
    }

    /// ### detect_collision
    /// used to see if a car is on a collision course with you
    pub fn detect_collision_course(&mut self, cars: &[Car]) {
        let mut distance = SCAN_AREA;
        for car in cars.iter().filter(|c| {
            self.id != c.id
                && self.vel < c.vel
                && self.crossing_paths(c)
                && self.calculate_distance(c) < SCAN_AREA
        }) {
            if self.calculate_distance(car) < distance {
                distance = self.calculate_distance(car);
            }
        }

        if distance < SCAN_AREA {
            self.brake(distance);
        }
    }

    fn crossing_paths(&self, other: &Car) -> bool {
        for sector in &self.path.sectors[self.index..=self.index + 2] {
            if sector.eq(&other.prev_sector()) || sector.eq(&other.get_sector()) {
                return true;
            }
        }
        false
    }

    pub fn ray_casting(&mut self, cars: &[Car]) {
        // Loop through all cars which are within collision range (one sector)
        let mut distance = SCAN_AREA;
        for car in cars.iter().filter(|c| {
            self.id != c.id
                && self.vel < c.vel
                && self.calculate_distance(c) < SCAN_AREA
                && self.moving != c.moving
        }) {
            // Only brake according to shortest distance
            if self.calculate_distance(car) > distance {
                continue;
            }
            let (x, y) = self.center_car();
            let (x2, y2) = car.center_car();
            match self.moving {
                Moving::Up => {
                    if y > y2 {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Down => {
                    if y < y2 {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Right => {
                    if x < x2 {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Left => {
                    if x > x2 {
                        distance = self.calculate_distance(car);
                    }
                }
            }
        }

        if distance < SCAN_AREA {
            self.brake(distance);
        }
    }

    /// ### leave_intersection
    /// used when cars are leaving the intersection, to adjust to the car in front to avoid
    /// weird looking "collisions".
    pub fn leave_intersection(&mut self, cars: &[Car]) {
        if let Some(car) = cars
            .iter()
            .find(|c| c.id < self.id && c.moving == self.moving)
        {
            if car.vel > self.vel {
                self.accelerate(self.calculate_distance(car));
            } else {
                self.brake(self.calculate_distance(car));
            }
        } else {
            self.accelerate(WINDOW_SIZE as f32);
        }
    }

    fn next_sector(&self) -> Sector {
        self.path.sectors[self.index + 1].clone()
    }

    fn prev_sector(&self) -> Sector {
        self.path.sectors[self.index - 1].clone()
    }

    /// ### adjacent_sectors
    /// Check for cars in the next. Also check adjacent sectors.
    ///
    /// **Calculation:**
    ///
    /// Up/Down: (x-1..=x+1).contains(x2) && y1 == y2
    /// Left/Right: (y-1..=y+1).contains(y2) && x1 == x2
    pub fn adjacent_sectors(&mut self, cars: &[Car]) {
        let sector = self.next_sector();
        let x = sector.get_x();
        let y = sector.get_y();

        let mut distance = SCAN_AREA;

        for car in cars.iter().filter(|c| {
            self.id != c.id
                && self.vel < c.vel
                && self.calculate_distance(c) < SCAN_AREA
                && self.direction != c.direction
        }) {
            let sector2 = car.get_sector();
            let x2 = sector2.get_x();
            let y2 = sector2.get_y();

            match self.moving {
                Moving::Up | Moving::Down => {
                    if (x - 1..=x + 1).contains(&x2) && y == y2 {
                        distance = self.calculate_distance(car);
                    }
                }
                Moving::Left | Moving::Right => {
                    if (y - 1..=y + 1).contains(&y2) && x == x2 {
                        distance = self.calculate_distance(car);
                    }
                }
            }
        }
        if distance < SCAN_AREA {
            self.brake(distance);
        }
    }

    /// ### center_car
    /// get the center point of a car
    pub fn center_car(&self) -> (f32, f32) {
        let top = self.get_borders().top;
        let right = self.get_borders().right;
        let bottom = self.get_borders().bottom;
        let left = self.get_borders().left;
        (left + ((right - left) / 2.0), top + ((bottom - top) / 2.0))
    }

    /// ### calculate_distance
    /// used to calculate the distance between two cars
    /// center points of both cars are used and then the distance formula:
    ///
    /// `sqrt(dx^2 + dy^2)`
    pub fn calculate_distance(&self, other: &Car) -> f32 {
        let (x, y) = self.center_car();
        let (x2, y2) = other.center_car();
        let dx = if x < x2 { x2 - x } else { x - x2 };
        let dy = if y < y2 { y2 - y } else { y - y2 };
        (dx * dx + dy * dy).sqrt()
    }
}
