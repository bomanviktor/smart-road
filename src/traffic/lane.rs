use crate::traffic::car::Car;
use crate::traffic::state::Direction;

type Lines = [[Option<Car>; 3]; 3];

#[derive(PartialEq)]
pub struct Lane {
    direction: Direction,
    lines: Lines
}

impl Lane {
    pub fn new(direction: Direction) -> Lane {
        Lane {
            direction,
            lines: generate_lines(),
        }
    }

    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car)
    }
}

fn generate_lines() -> Lines {
    [
        [None, None, None],
        [None, None, None],
        [None, None, None]
    ]
}
