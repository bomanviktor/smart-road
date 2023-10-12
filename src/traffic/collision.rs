use crate::config::{
    ACCELERATION_DISTANCE, CRUISE_SPEED, MARGIN, SCAN_DISTANCE, SECTOR_WIDTH, WINDOW_SIZE,
};
use crate::traffic::*;

impl Car {
    /// ### forward_scan
    /// Scans the sectors in front of the car and accelerate depending on the distance
    /// to the closest car in front
    pub fn forward_scan(&mut self, cars: &[Car]) {
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
            let (self_x, self_y) = self.center_car();
            match self.moving {
                Moving::Up => {
                    if y < self_y && scan_x.contains(&x) {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Down => {
                    if y > self_y && scan_x.contains(&x) {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Right => {
                    if x > self_x && scan_y.contains(&y) {
                        distance = self.calc_dist(car);
                    }
                }
                Moving::Left => {
                    if x < self_x && scan_y.contains(&y) {
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

    /// ### ray_casting
    /// Check if there are any cars in front of self are inside the `SCAN_DISTANCE`.
    /// If these cars have a shorter distance to the exit than self, brake according to the closest
    /// of these cars.
    pub fn ray_casting(&mut self, cars: &[Car]) {
        // Loop through all cars which are within collision range (one sector)
        let mut distance = SCAN_DISTANCE;
        for car in cars.iter().filter(|c| {
            self.longer_distance_to_exit(c)
                && self.calc_dist(c) < SCAN_DISTANCE
                && self.crossing_paths(c)
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

        if distance < SCAN_DISTANCE {
            self.brake(distance);
        }
    }

    pub fn check_passing(&mut self, cars: &[Car]) {
        let index = if self.turning == Turning::Straight {
            6..=8
        } else {
            5..=7
        };
        if cars.iter().any(|c| {
            c.id != self.id
                && c.turning == Turning::Straight
                && self.direction != c.direction
                && self.calc_dist(c) < SCAN_DISTANCE
                && index.contains(&c.index)
        }) {
            self.stop();
        }
    }

    pub fn sector_in_front(&mut self, cars: &[Car]) {
        if let Some(car) = cars
            .iter()
            .find(|c| c.id != self.id && self.sector(1).eq(&c.sector(0)))
        {
            self.brake(self.calc_dist(car));
        }
    }

    /// ### crossing_paths
    /// Check if a car has a crossing path with self
    fn crossing_paths(&self, other: &Car) -> bool {
        for sector in &self.path.sectors[self.index..=self.index + 2] {
            if sector.eq(&other.sector(1))
                || (sector.eq(&other.sector(0)) && other.sector_pos() < SECTOR_WIDTH / 2.0)
            {
                return true;
            }
        }
        false
    }

    /// ### longer_distance_to_exit
    /// Check if `self` has a longer distance to the exit than `other`
    fn longer_distance_to_exit(&self, other: &Car) -> bool {
        self.path.sectors.len() as f32 * SECTOR_WIDTH
            - (self.index as f32 * SECTOR_WIDTH + self.sector_pos())
            > other.path.sectors.len() as f32 * SECTOR_WIDTH
                - (other.index as f32 * SECTOR_WIDTH + other.sector_pos())
    }

    pub fn center_scan(&mut self, cars: &[Car]) {
        if cars
            .iter()
            .any(|c| self.id < c.id && (5..=7).contains(&c.index) && c.turning == Turning::Left)
        {
            self.vel = CRUISE_SPEED;
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
        let (dx, dy) = ((x - x2).abs(), (y - y2).abs());
        (dx * dx + dy * dy).sqrt()
    }
}
