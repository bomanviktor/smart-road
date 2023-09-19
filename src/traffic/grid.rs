use crate::traffic::car::Car;
use crate::traffic::path::Sector;

type Intersection = [[Option<Car>; 6]; 6];

#[derive(Clone, PartialEq)]
pub struct Grid {
    intersection: Intersection, // None if no car, Some if there is a car
    occupied_sectors: Vec<Sector>,
    empty_sectors: Vec<Sector>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            intersection: new_intersection(),
            occupied_sectors: Vec::new(),
            empty_sectors: Vec::new(),
        }
    }

    pub fn update_intersection(&mut self) {
        self.occupied_sectors.clear();
        self.empty_sectors.clear();
        for (y, cars) in self.intersection.iter().enumerate() {
            for (x, car) in cars.iter().enumerate() {
                if car.is_none() {
                    self.empty_sectors.push(Sector::new(x, y));
                } else {
                    self.occupied_sectors.push(Sector::new(x, y));
                }
            }
        }
    }

    pub fn insert_car_to_intersection(&mut self, car: Car) {
        let i = car.path.current;
        let x = car.path.sectors[i].get_x();
        let y = car.path.sectors[i].get_y();
        self.intersection[x - 3][y - 3] = Some(car);
    }

    pub fn get_intersection(&self) -> &Intersection {
        &self.intersection
    }
    pub fn get_occupied(&self) -> &Vec<Sector> {
        &self.occupied_sectors
    }
    pub fn get_empty(&self) -> &Vec<Sector> {
        &self.empty_sectors
    }
}

fn new_intersection() -> Intersection {
    [
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
        [None, None, None, None, None, None],
    ]
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
