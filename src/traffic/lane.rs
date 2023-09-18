use crate::traffic::car::Car;
use crate::traffic::Turning;

type Lines = [[Option<Car>; 3]; 3];
#[derive(PartialEq)]
pub struct Lane {
    cars: Vec<Car>,
    lines: Lines,
}

impl Lane {
    pub fn new() -> Lane {
        Lane {
            cars: Vec::new(),
            lines: generate_lines(),
        }
    }

    pub fn add_car(&mut self, car: Car) {
        match car.turning {
            Turning::Left => self.lines[0][0] = Some(car.clone()),
            Turning::Straight => self.lines[1][0] = Some(car.clone()),
            Turning::Right => self.lines[2][0] = Some(car.clone()),
        }
        self.cars.push(car)
    }

    pub fn available_lines(&self) -> Vec<Turning> {
        let available: Vec<bool> = self
            .lines
            .iter()
            .map(|line| line[0].is_none() && line[1].is_none())
            .collect();

        let mut available_lines = Vec::new();
        for (i, l) in available.into_iter().enumerate() {
            match i {
                0 => {
                    if l {
                        available_lines.push(Turning::Left);
                    }
                }
                1 => {
                    if l {
                        available_lines.push(Turning::Straight);
                    }
                }
                _ => {
                    if l {
                        available_lines.push(Turning::Right);
                    }
                }
            }
        }
        available_lines
    }
}

fn generate_lines() -> Lines {
    [[None, None, None], [None, None, None], [None, None, None]]
}

impl Default for Lane {
    fn default() -> Self {
        Self::new()
    }
}
