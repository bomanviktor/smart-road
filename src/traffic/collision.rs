use crate::traffic::*;

impl Car {
    /// ### scan_in_front
    /// Scans the sector in front of car.
    /// Uses 5 px margin to avoid scanning unwanted cars.
    /// ### scan_in_front
    /// Scans the sector in front of car.
    /// Uses 5 px margin to avoid scanning unwanted cars.
    pub fn scan_in_front(&mut self, cars: &[Car]) -> bool {
        let left = self.get_borders().left;
        let right = self.get_borders().right;
        let top = self.get_borders().top;
        let bottom = self.get_borders().bottom;

        let mut shortest = 0.0;

        for car in cars.iter().filter(|c| c.id != self.id) {
            let (car_x, car_y) = car.center_car();

            match self.moving {
                Moving::Up => {
                    if car_y > self.center_car().1 {
                        continue;
                    }
                    if (left..right).contains(&car_x) && self.calculate_distance(car) > shortest {
                        shortest = self.calculate_distance(car);
                    }
                }
                Moving::Down => {
                    if car_y < self.center_car().1 {
                        continue;
                    }
                    if (left..right).contains(&car_x) && self.calculate_distance(car) > shortest {
                        shortest = self.calculate_distance(car);
                    }
                }
                Moving::Right => {
                    if car_x < self.center_car().0 {
                        continue;
                    }
                    if (top..bottom).contains(&car_y) && self.calculate_distance(car) > shortest {
                        shortest = self.calculate_distance(car);
                    }
                }
                Moving::Left => {
                    if car_x > self.center_car().0 {
                        continue;
                    }
                    if (top..bottom).contains(&car_y) && self.calculate_distance(car) > shortest {
                        shortest = self.calculate_distance(car);
                    }
                }
            }
        }

        if shortest != 0.0 {
            self.brake(shortest);
        }
        false
    }

    /*fn opposite_direction(&self, other: &Car) -> bool {
        match self.moving {
            Moving::Up => other.moving == Moving::Down,
            Moving::Down => other.moving == Moving::Up,
            Moving::Left => other.moving == Moving::Right,
            Moving::Right => other.moving == Moving::Left,
        }
    }

     */

    pub fn center_car(&self) -> (f32, f32) {
        let top = self.get_borders().top;
        let right = self.get_borders().right;
        let bottom = self.get_borders().bottom;
        let left = self.get_borders().left;
        (right - left / 2.0, bottom - top / 2.0)
    }

    pub fn sector_ahead(&mut self, cars: &[Car]) {
        for car in cars {
            if car.get_sector() == self.next_sector() {
                if self.moving == car.moving {
                    if self.vel > car.vel {
                        self.vel = car.vel;
                    } else {
                        self.accelerate();
                    }
                } else {
                    self.brake(self.calculate_distance(car));
                }
            }
        }
    }

    pub fn break_deadlock(&mut self, cars: &[Car]) {
        for car in cars.iter().filter(|c| c.vel == 0.0 && self.neighbors(c)) {
            if self.index > car.index {
                self.accelerate();
                return;
            }
        }
    }

    fn neighbors(&self, other: &Car) -> bool {
        let x = self.get_sector().get_x();
        let y = self.get_sector().get_y();
        (x - 1..=x + 1).contains(&other.get_sector().get_x())
            && (y - 1..=y + 1).contains(&other.get_sector().get_y())
    }

    fn next_sector(&self) -> Sector {
        self.path.sectors[self.index + 1].clone()
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
    pub fn adjacent_sectors(&mut self, grid: &Grid, sectors_ahead: usize) -> Option<Car> {
        let i = self.index;
        let sector_ahead = &self.path.sectors[i + sectors_ahead];
        let x = sector_ahead.get_x();
        let y = sector_ahead.get_y();

        match sector_ahead.moving {
            Moving::Up | Moving::Down => {
                if let Some(car) = grid.get_car_at_coords(x - sectors_ahead, y) {
                    if car.moving == Moving::Left && self.sector_position() < car.sector_position()
                    {
                        return Some(car);
                    }
                }
                if grid.get_car_at_coords(x, y).is_some() {
                    return grid.get_car_at_coords(x, y);
                }
                if let Some(car) = grid.get_car_at_coords(x + sectors_ahead, y) {
                    if car.moving == Moving::Right && self.sector_position() < car.sector_position()
                    {
                        return Some(car);
                    }
                }
            }
            Moving::Right | Moving::Left => {
                if let Some(car) = grid.get_car_at_coords(x, y - sectors_ahead) {
                    if car.moving == Moving::Down && self.sector_position() < car.sector_position()
                    {
                        return Some(car);
                    }
                }
                if grid.get_car_at_coords(x, y).is_some() {
                    return grid.get_car_at_coords(x, y);
                }
                if let Some(car) = grid.get_car_at_coords(x, y + sectors_ahead) {
                    if car.moving == Moving::Up && self.sector_position() < car.sector_position() {
                        return Some(car);
                    }
                }
            }
        }
        None
    }

    pub fn calculate_distance(&self, other: &Car) -> f32 {
        let dx = if self.center_car().0 < other.center_car().0 {
            other.center_car().0 - self.center_car().0
        } else {
            self.center_car().0 - other.center_car().0
        };

        let dy = if self.center_car().1 < other.center_car().1 {
            other.center_car().1 - self.center_car().1
        } else {
            self.center_car().1 - other.center_car().1
        };

        (dx * dx + dy * dy).sqrt()
    }
}
