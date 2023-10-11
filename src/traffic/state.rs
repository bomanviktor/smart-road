use crate::config::{CLOSE_CALL_DISTANCE, COLLISION_DISTANCE, MARGIN, SECTOR_WIDTH};
use macroquad::rand::gen_range;

use crate::traffic::car::Car;
use crate::traffic::road::Road;
use crate::traffic::statistics::*;
use crate::traffic::Turning;

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
    pub stats: Statistics,
    pub show_final_statistics: bool,
    pub random: bool,
    pub total_cars: usize,
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
            stats: Statistics::default(),
            random: false,
            total_cars: 0,
            show_final_statistics: false,
        }
    }

    pub fn update(&mut self) {
        let all_cars = self.get_all_cars();

        self.roads.iter_mut().for_each(|road| {
            // Cleanup and statistics logic
            road.cleanup_cars(&mut self.stats);

            // Iterating over each lane's cars
            road.cars.iter_mut().for_each(|cars| {
                cars.iter_mut().for_each(|car| {
                    if detect_collision(car, &all_cars) {
                        self.stats.set_collisions()
                    } else if detect_close_call(car, &all_cars) {
                        self.stats.set_close_calls();
                    }

                    if detect_deadlock(&all_cars, car) {
                        car.stop();
                        return;
                    }
                    self.stats.set_velocity(car.vel);
                    car.move_car(&all_cars);
                });
            });
        });
    }
    pub fn add_car(&mut self, direction: Direction) {
        if self.get_all_cars().iter().filter(|c| c.vel == 0.0).count() >= 8 {
            return;
        }
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

    pub fn get_all_cars(&self) -> Vec<Car> {
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

fn detect_close_call(car: &Car, other_cars: &[Car]) -> bool {
    other_cars
        .iter()
        .any(|c| c.id != car.id && car.calc_dist(c) <= CLOSE_CALL_DISTANCE)
}

fn detect_collision(car: &Car, other_cars: &[Car]) -> bool {
    other_cars
        .iter()
        .any(|c| c.id != car.id && car.calc_dist(c) <= COLLISION_DISTANCE)
}

fn detect_deadlock(other_cars: &[Car], car: &mut Car) -> bool {
    if car.turning != Turning::Left {
        return false;
    }

    let middle_sectors = [(5, 5), (5, 6), (6, 5), (6, 6)];
    let cars: Vec<&Car> = other_cars
        .iter()
        .filter(|&c| middle_sectors.contains(&(c.sector(0).get_x(), c.sector(0).get_y())))
        .collect();

    if car.index == 3 && car.sector_pos() > SECTOR_WIDTH - MARGIN {
        return cars.len() >= 2;
    }

    if car.index == 4 && car.sector_pos() > SECTOR_WIDTH - MARGIN {
        let north = cars
            .iter()
            .filter(|c| c.direction == Direction::North)
            .count();
        let east = cars
            .iter()
            .filter(|c| c.direction == Direction::East)
            .count();
        let south = cars
            .iter()
            .filter(|c| c.direction == Direction::South)
            .count();
        let west = cars
            .iter()
            .filter(|c| c.direction == Direction::West)
            .count();
        match car.direction {
            Direction::West => {
                if north >= 2 || east >= 2 || south >= 2 {
                    return false;
                }
            }
            Direction::South => {
                if north >= 2 || east >= 2 || west >= 2 {
                    return false;
                }
            }
            Direction::North => {
                if west >= 2 || east >= 2 || south >= 2 {
                    return false;
                }
            }
            Direction::East => {
                if north >= 2 || west >= 2 || south >= 2 {
                    return false;
                }
            }
        }
        return cars.len() >= 3;
    }
    false
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
