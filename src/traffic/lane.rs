use crate::traffic::car::Car;
use crate::traffic::state::Direction;

#[derive(PartialEq)]
pub struct Lane {
    direction: Direction,
    pub cars: Vec<Car>,
}

impl Lane {
    pub fn new(direction: Direction) -> Lane {
        Lane {
            direction,
            cars: Vec::new(),
        }
    }

    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car)
    }
}
