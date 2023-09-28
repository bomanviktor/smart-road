use macroquad::rand::gen_range;

use crate::traffic::car::Car;
use crate::traffic::grid::Grid;
use crate::traffic::road::Road;
use crate::traffic::statistics::*;

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
    pub paused: bool,
    pub random: bool,
    pub display_grid: bool,
    total_cars: usize,
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
            paused: false,
            random: false,
            display_grid: false,
            total_cars: 1,
        }
    }

    pub fn prevent_deadlock(&self) -> bool {
        let middle_sectors = vec![(5, 5), (5, 6), (6, 5), (6, 6)];
        self
            .get_all_cars()
            .iter()
            .filter(|c| {
                middle_sectors.contains(&(c.get_sector().get_x(), c.get_sector().get_y()))
            })
            .count() >= 3
    }

    pub fn update(&mut self) {
        let all_cars = self.get_all_cars();
        let potential_deadlock = self.prevent_deadlock(); // Call to our new deadlock detection method

        self.roads.iter_mut().for_each(|road| {
            // Cleanup and statistics logic
            road.cleanup_cars(&mut self.stats);

            // Iterating over each lane's cars
            road.cars.iter_mut().for_each(|cars| {
                cars.iter_mut().for_each(|car| {
                    // Check if the car should stop to prevent deadlock
                    if car.should_stop(potential_deadlock) && car.index < 5 {
                        car.stop(); // This will set car's velocity to 0
                        return;
                    }

                    // Existing car movement and statistics logic
                    self.stats.set_velocity(car.vel);
                    car.move_car(&all_cars);

                    // Update the grid based on the car's new state
                    self.grid.update_grid(car.clone());
                });
            });
        });

        // Refresh the grid after all updates
        self.grid.refresh_grid();
    }
    pub fn add_car(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                let available_path = self.roads[0].get_available_path();
                if let Some(path) = available_path {
                    self.roads[0].add_car(Car::new(direction, path, self.total_cars));
                    self.total_cars += 1;
                }
            }
            Direction::East => {
                let available_path = self.roads[1].get_available_path();

                if let Some(path) = available_path {
                    self.roads[1].add_car(Car::new(direction, path, self.total_cars));
                    self.total_cars += 1;
                }
            }
            Direction::South => {
                let available_path = self.roads[2].get_available_path();
                if let Some(path) = available_path {
                    self.roads[2].add_car(Car::new(direction, path, self.total_cars));
                    self.total_cars += 1;
                }
            }
            Direction::West => {
                let available_path = self.roads[3].get_available_path();
                if let Some(path) = available_path {
                    self.roads[3].add_car(Car::new(direction, path, self.total_cars));
                    self.total_cars += 1;
                }
            }
        }
    }

    fn get_all_cars(&self) -> Vec<Car> {
        let mut cars = Vec::new();
        for r in self.roads.iter() {
            for car in r.cars.clone().iter().take(2).flatten() {
                if (1..11).contains(&car.index) {
                    cars.push(car.clone());
                }
            }
        }

        cars
    }

    pub fn add_car_random(&mut self) {
        match gen_range(0, 4) {
            0 => self.add_car(Direction::North),
            1 => self.add_car(Direction::East),
            2 => self.add_car(Direction::South),
            _ => self.add_car(Direction::West),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
