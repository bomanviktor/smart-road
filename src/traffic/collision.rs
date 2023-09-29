use crate::config::{ACCELERATION_DISTANCE, MARGIN, SCAN_AREA, SECTOR_WIDTH, WINDOW_SIZE};
use crate::traffic::*;

impl Car {
    /// ### forward_scan
    /// Scans the sectors in front of the car and accelerate depending on the distance
    /// to the closest car in front
    pub fn forward_scan(&mut self, cars: &[Car]) {
        let borders = self.borders();
        let top = borders.top;
        let bottom = borders.bottom;
        let left = borders.left;
        let right = borders.right;

        // Get the ranges where we scan cars in front
        let scan_x = self.borders().left + MARGIN..=self.borders().right - MARGIN;
        let scan_y = self.borders().top + MARGIN..=self.borders().bottom - MARGIN;

        // The longest distance to car in front.
        let mut distance = WINDOW_SIZE as f32;
        for car in cars.iter().filter(|c| c.id != self.id) {
            if self.calc_dist(car) > distance {
                continue;
            }

            let (x, y) = car.center_car();
            match self.moving {
                Moving::Up => {
                    if y < top && scan_x.contains(&x) {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Down => {
                    if y > bottom && scan_x.contains(&x) {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Right => {
                    if x > right && scan_y.contains(&y) {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Left => {
                    if x < left && scan_y.contains(&y) {
                        distance = self.calc_dist(car);
                    }
                }
            }
        }
        if distance > ACCELERATION_DISTANCE {
            self.accelerate(distance);
        } else {
            self.brake(distance);
        }
    }

    pub fn no_adjacent(&self, cars: &[Car]) -> bool {
        cars.iter().any(|c| !(3..=4).contains(&c.index))
    }
    fn crossing_paths(&self, other: &Car) -> bool {
        for sector in &self.path.sectors[self.index..=self.index + 2] {
            if sector.eq(&other.sector(-1))
                || (sector.eq(&other.sector(0)) && other.sector_position() < SECTOR_WIDTH / 2.0)
            {
                return true;
            }
        }
        false
    }

    pub fn ray_casting(&mut self, cars: &[Car]) {
        // Loop through all cars which are within collision range (one sector)
        let mut distance = SCAN_AREA;
        for car in cars.iter().filter(|c| {
            self.id < c.id
                && self.calc_dist(c) < SCAN_AREA
                && self.crossing_paths(c)
                && self.moving != c.moving
        }) {
            // Only brake according to shortest distance
            if self.calc_dist(car) > distance {
                continue;
            }

            let (x, y) = self.center_car();
            let (x2, y2) = car.center_car();

            match self.moving {
                Moving::Up => {
                    if y > y2 {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Down => {
                    if y < y2 {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Right => {
                    if x < x2 {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Left => {
                    if x > x2 {
                        distance = self.calc_dist(car);
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
                self.accelerate(self.calc_dist(car));
            } else {
                self.brake(self.calc_dist(car));
            }
        } else {
            self.accelerate(WINDOW_SIZE as f32);
        }
    }

    /// ### adjacent_sectors
    /// Check for cars in the next sector. Also check adjacent sectors.
    ///
    /// **Calculation:**
    ///
    /// Up/Down: (x-1..=x+1).contains(x2) && y1 == y2
    /// Left/Right: (y-1..=y+1).contains(y2) && x1 == x2
    pub fn adjacent_sectors(&mut self, cars: &[Car]) {
        let sector = self.sector(1);
        let x = sector.get_x();
        let y = sector.get_y();

        let mut distance = SCAN_AREA;

        for car in cars.iter().filter(|c| {
            self.id > c.id && self.calc_dist(c) < SCAN_AREA && self.direction != c.direction
        }) {
            let sector2 = car.sector(0);
            let x2 = sector2.get_x();
            let y2 = sector2.get_y();

            match self.moving {
                Moving::Up | Moving::Down => {
                    if (x - 1..=x + 1).contains(&x2) && y == y2 {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Left | Moving::Right => {
                    if (y - 1..=y + 1).contains(&y2) && x == x2 {
                        distance = self.calc_dist(car);
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
        let top = self.borders().top;
        let right = self.borders().right;
        let bottom = self.borders().bottom;
        let left = self.borders().left;
        (left + ((right - left) / 2.0), top + ((bottom - top) / 2.0))
    }

    /// ### calculate_distance
    /// used to calculate the distance between two cars
    /// center points of both cars are used and then the distance formula:
    ///
    /// `sqrt(dx^2 + dy^2)`
    pub fn calc_dist(&self, other: &Car) -> f32 {
        let (x, y) = self.center_car();
        let (x2, y2) = other.center_car();
        let dx = if x < x2 { x2 - x } else { x - x2 };
        let dy = if y < y2 { y2 - y } else { y - y2 };
        (dx * dx + dy * dy).sqrt()
    }
}
