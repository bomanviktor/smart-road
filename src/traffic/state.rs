use crate::traffic::car::Car;
use crate::traffic::grid::Grid;
use crate::traffic::road::Road;
use crate::traffic::statistics::*;
use macroquad::rand::gen_range;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Debug)]
pub struct State {
    pub roads: [Road; 4],
    pub grid: Grid,
    pub stats: Statistics,
}

impl State {
    pub fn new() -> State {
        State {
            roads: [
                Road::new(Direction::North),
                Road::new(Direction::East),
                Road::new(Direction::South),
                Road::new(Direction::West),
            ],
            grid: Grid::default(),
            stats: Statistics::default(),
        }
    }

    pub fn update(&mut self) {
        self.roads.iter_mut().for_each(|road| {
            // Clean up finished cars and add their time to stats.
            road.cleanup_cars(&mut self.stats);

            // Get all cars from all paths from each road.
            road.cars.iter_mut().for_each(|cars| {
                // Update x and y for each car, and update velocity statistics.
                cars.iter_mut().for_each(|car| {
                    self.stats.set_velocity(car.get_velocity());
                    car.move_car();
                    println!("{}", car);
                })
            });
        });

        self.grid.get_intersection();
        self.grid.update_intersection();
        self.grid.get_empty();
        self.grid.get_occupied();
    }

    pub fn add_car(&mut self, direction: Direction) {
        let sprite_index = gen_range(0, 6);
        match direction {
            Direction::North => {
                let available_path = self.roads[0].get_available_path();

                if let Some(path) = available_path {
                    self.roads[0].add_car(Car::new(direction, path, sprite_index));
                }
            }
            Direction::East => {
                let available_path = self.roads[1].get_available_path();

                if let Some(path) = available_path {
                    self.roads[1].add_car(Car::new(direction, path, sprite_index));
                }
            }
            Direction::South => {
                let available_path = self.roads[2].get_available_path();

                if let Some(path) = available_path {
                    self.roads[2].add_car(Car::new(direction, path, sprite_index));
                }
            }
            Direction::West => {
                let available_path = self.roads[3].get_available_path();

                if let Some(path) = available_path {
                    self.roads[3].add_car(Car::new(direction, path, sprite_index));
                }
            }
        }
    }

    pub fn add_car_random(&mut self) {
        match gen_range(0, 4) {
            0 => self.add_car(Direction::North),
            1 => self.add_car(Direction::East),
            2 => self.add_car(Direction::South),
            _ => self.add_car(Direction::West),
        }
    }

    pub fn update_grid(&mut self) {
        self.roads.iter_mut().for_each(|road| {
            road.cars.iter_mut().for_each(|cars| {
                cars.iter_mut().for_each(|car| {
                    car.update_in_grid(&mut self.grid);
                })
            })
        })
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
