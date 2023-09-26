use crate::config::*;
use crate::traffic::*;
impl Car {
    /// ### scan_in_front
    /// Scans the sector in front of car.
    /// Uses 5 px margin to avoid scanning unwanted cars.
    /// ### scan_in_front
    /// Scans the sector in front of car.
    /// Uses 5 px margin to avoid scanning unwanted cars.
    pub fn scan_in_front(&mut self, cars: &[Car]) -> bool {
        let top = self.get_borders().top as usize;
        let right = self.get_borders().right as usize;
        let bottom = self.get_borders().bottom as usize;
        let left = self.get_borders().left as usize;

        for car in cars {
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
                        for y in top - 20..bottom + 20 {
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

    pub fn center_car(&self) -> usize {
        let top = self.get_borders().top as usize;
        let right = self.get_borders().right as usize;
        let bottom = self.get_borders().bottom as usize;
        let left = self.get_borders().left as usize;
        match self.moving {
            Moving::Right | Moving::Left => bottom - top / 2,
            Moving::Down | Moving::Up => right - left / 2,
        }
    }

    pub fn sector_ahead(&mut self, cars: &[Car]) {
        for car in cars {
            if car.next_sector() == self.next_sector()
                && car.sector_position() > self.sector_position()
            {
                self.slow_down();
                self.brake()
            }
        }
    }

    fn next_sector(&self) -> Sector {
        self.path.sectors[self.path.current].clone()
    }

    /// ### scan_sector
    /// Scans the current sector and checks if another car is inside the sector.
    /* fn scan_sector(&self, cars: &[Car]) -> bool {
            for car in cars.iter().filter(|&c| {
                self.get_sector() == c.get_sector()

            }) {
                if self.sector_position() < car.sector_position() {
                    return true;
                }
            }
            false
        }
    */

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
    pub fn adjacent_sectors(&mut self, grid: &Grid, sectors_ahead: usize) -> bool {
        let i = self.path.current;
        let sector_ahead = &self.path.sectors[i + sectors_ahead];
        let x = sector_ahead.get_x();
        let y = sector_ahead.get_y();

        match sector_ahead.moving {
            Moving::Up | Moving::Down => {
                if let Some(car) = grid.get_car_at_coords(x - sectors_ahead, y) {
                    if car.moving == Moving::Right {
                        return self.sector_position() < car.sector_position();
                    }
                }
                if grid.get_car_at_coords(x, y).is_some() {
                    return true;
                }
                if let Some(car) = grid.get_car_at_coords(x + sectors_ahead, y) {
                    if car.moving == Moving::Left {
                        return self.sector_position() < car.sector_position();
                    }
                }
            }
            Moving::Right | Moving::Left => {
                if let Some(car) = grid.get_car_at_coords(x, y - sectors_ahead) {
                    if car.moving == Moving::Down {
                        return self.sector_position() < car.sector_position();
                    }
                }
                if grid.get_car_at_coords(x, y).is_some() {
                    return true;
                }
                if let Some(car) = grid.get_car_at_coords(x, y + sectors_ahead) {
                    if car.moving == Moving::Up {
                        return self.sector_position() < car.sector_position();
                    }
                }
            }
        }
        false
    }
}
