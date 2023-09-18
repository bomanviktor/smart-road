use crate::traffic::car::Car;
use crate::traffic::grid::Grid;
use crate::traffic::road::Road;
use crate::traffic::statistics::*;
use macroquad::rand::gen_range;
use rand::prelude::IteratorRandom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq)]
pub struct State {
    pub roads: [Road; 4],
    pub grid: Grid,
    pub stats: Statistics,
}

impl State {
    pub fn new() -> State {
        State {
            roads: [
                Road::default(),
                Road::default(),
                Road::default(),
                Road::default(),
            ],
            grid: Grid::default(),
            stats: Statistics::default(),
        }
    }

    pub fn update(&mut self) {
        self.roads.iter_mut().for_each(|road| {
            // Clean up finished cars and add their time to stats.
            road.cleanup_cars(&mut self.stats);

            // Add velocity to stats and move car.
            road.cars.iter_mut().for_each(|car| {
                self.stats.set_velocity(car.get_velocity());
                car.move_car();
            })
        });

        self.grid.get_intersection();
        self.grid.update_intersection();
        self.grid.get_empty();
        self.grid.get_occupied();
    }

    pub fn add_car(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                let available_lanes = self.roads[0]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(lane) = available_lanes {
                    self.roads[0].add_car(Car::new(direction, lane));
                }
            }
            Direction::East => {
                let available_lanes = self.roads[1]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(lane) = available_lanes {
                    self.roads[1].add_car(Car::new(direction, lane));
                }
            }
            Direction::South => {
                let available_lanes = self.roads[2]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(lane) = available_lanes {
                    self.roads[2].add_car(Car::new(direction, lane));
                }
            }
            Direction::West => {
                let available_lanes = self.roads[3]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(lane) = available_lanes {
                    self.roads[3].add_car(Car::new(direction, lane));
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
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
