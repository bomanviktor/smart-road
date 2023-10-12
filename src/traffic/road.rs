use rand::prelude::IteratorRandom;

use crate::traffic::car::Car;
use crate::traffic::{Direction, Statistics, Turning};
#[derive(PartialEq, Debug, Clone)]
pub struct Road {
    direction: Direction,
    pub cars: [Vec<Car>; 3],
}

impl Road {
    pub fn new(direction: Direction) -> Road {
        Road {
            direction,
            cars: [Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn add_car(&mut self, car: Car) {
        match car.turning {
            Turning::Left => self.cars[0].push(car.clone()),
            Turning::Straight => self.cars[1].push(car.clone()),
            Turning::Right => self.cars[2].push(car.clone()),
        }
    }

    pub fn get_available_path(&self) -> Option<Turning> {
        let lanes = self.available_lanes();
        let mut paths = Vec::new();

        for (i, available) in lanes.into_iter().enumerate() {
            match i {
                0 => {
                    if available {
                        paths.push(Turning::Left);
                    }
                }
                1 => {
                    if available {
                        paths.push(Turning::Straight);
                    }
                }
                _ => {
                    if available {
                        paths.push(Turning::Right);
                    }
                }
            }
        }

        paths.into_iter().choose(&mut rand::thread_rng())
    }

    fn available_lanes(&self) -> [bool; 3] {
        let mut available = [false, false, false];

        if self.cars[0].is_empty() {
            available[0] = true;
        } else {
            let prev_car = self.cars[0].iter().next_back().unwrap();
            if prev_car.index > 2 {
                available[0] = true;
            }
        }

        if self.cars[1].is_empty() {
            available[1] = true;
        } else {
            let prev_car = self.cars[1].iter().next_back().unwrap();
            if prev_car.index > 2 {
                available[1] = true;
            }
        }

        if self.cars[2].is_empty() {
            available[2] = true;
        } else {
            let prev_car = self.cars[2].iter().next_back().unwrap();
            if prev_car.index > 2 {
                available[2] = true;
            }
        }

        available
    }

    // Add time for all cars that reached their destination and then remove from vector.
    pub fn cleanup_cars(&mut self, stats: &mut Statistics) {
        self.cars.iter().for_each(|cars| {
            cars.iter()
                .filter(|car| car.is_done())
                .for_each(|car| car.add_time(stats))
        });

        self.cars[0].retain(|car| !car.is_done());
        self.cars[1].retain(|car| !car.is_done());
        self.cars[2].retain(|car| !car.is_done());
    }
}
