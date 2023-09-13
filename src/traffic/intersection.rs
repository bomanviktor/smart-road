use crate::traffic::car::Car;
use crate::traffic::path::Sector;

type Grid = [[Option<Car>; 6];6];

#[derive(Clone)]
pub struct Intersection {
    grid: Grid, // None if no car, Some if there is a car
    occupied_sectors: Vec<Sector>,
    empty_sectors: Vec<Sector>
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            grid: new_grid(),
            occupied_sectors: Vec::new(),
            empty_sectors: Vec::new()
        }
    }

    pub fn update_grid(&mut self) {
        self.occupied_sectors.clear();
        self.empty_sectors.clear();
        for (y, cars) in self.grid.iter().enumerate() {
            for (x, car) in cars.iter().enumerate() {
                if car.is_none() {
                    self.empty_sectors.push(Sector::new(x, y));
                } else {
                    self.occupied_sectors.push(Sector::new(x, y));
                }
            }
        }
    }

    pub fn insert_car_to_grid(&mut self, car: Car) {
        let x = car.path.sectors[0].get_x();
        let y = car.path.sectors[0].get_y();
        self.grid[x][y] = Some(car);
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
    pub fn get_occupied(&self) -> &Vec<Sector> {
        &self.occupied_sectors
    }
    pub fn get_empty(&self) -> &Vec<Sector> {
        &self.empty_sectors
    }
}

fn new_grid() -> Grid {
    [
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
    ]
}