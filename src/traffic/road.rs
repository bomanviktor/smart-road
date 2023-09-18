use crate::traffic::car::Car;
use crate::traffic::{Statistics, Turning};

type Lanes = [[Option<Car>; 3]; 3];
#[derive(PartialEq)]
pub struct Road {
    pub cars: Vec<Car>,
    lanes: Lanes,
}

impl Road {
    pub fn new() -> Road {
        Road {
            cars: Vec::new(),
            lanes: generate_lanes(),
        }
    }

    pub fn add_car(&mut self, car: Car) {
        match car.turning {
            Turning::Left => self.lanes[0][0] = Some(car.clone()),
            Turning::Straight => self.lanes[1][0] = Some(car.clone()),
            Turning::Right => self.lanes[2][0] = Some(car.clone()),
        }
        self.cars.push(car)
    }

    pub fn available_lines(&self) -> Vec<Turning> {
        let available: Vec<bool> = self
            .lanes
            .iter()
            .map(|lane| lane[0].is_none() && lane[1].is_none())
            .collect();

        let mut available_lanes = Vec::new();
        for (i, l) in available.into_iter().enumerate() {
            match i {
                0 => {
                    if l {
                        available_lanes.push(Turning::Left);
                    }
                }
                1 => {
                    if l {
                        available_lanes.push(Turning::Straight);
                    }
                }
                _ => {
                    if l {
                        available_lanes.push(Turning::Right);
                    }
                }
            }
        }
        available_lanes
    }

    // Add time for all cars that reached their destination and then remove from vector.
    pub fn cleanup_cars(&mut self, stats: &mut Statistics) {
        self.cars
            .iter()
            .filter(|car| car.is_done())
            .for_each(|car| car.add_time(stats));

        self.cars.retain(|car| !car.is_done());
    }
}

fn generate_lanes() -> Lanes {
    [[None, None, None], [None, None, None], [None, None, None]]
}

impl Default for Road {
    fn default() -> Self {
        Self::new()
    }
}
