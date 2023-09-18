use crate::traffic::car::Car;
use crate::traffic::grid::Grid;
use crate::traffic::lane::Lane;
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
    pub lanes: [Lane; 4],
    pub grid: Grid,
    pub stats: Statistics,
}

impl State {
    pub fn new() -> State {
        State {
            lanes: [
                Lane::default(),
                Lane::default(),
                Lane::default(),
                Lane::default(),
            ],
            grid: Grid::default(),
            stats: Statistics::default(),
        }
    }

    pub fn update(&mut self) {
        self.grid.get_intersection();
        self.grid.update_intersection();
        self.grid.get_empty();
        self.grid.get_occupied();
    }

    pub fn add_car(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                let available_lines = self.lanes[0]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(line) = available_lines {
                    self.lanes[0].add_car(Car::new(Direction::North, line));
                }
            }
            Direction::East => {
                let available_lines = self.lanes[1]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(line) = available_lines {
                    self.lanes[1].add_car(Car::new(Direction::North, line));
                }
            }
            Direction::South => {
                let available_lines = self.lanes[2]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(line) = available_lines {
                    self.lanes[2].add_car(Car::new(Direction::North, line));
                }
            }
            Direction::West => {
                let available_lines = self.lanes[3]
                    .available_lines()
                    .into_iter()
                    .choose(&mut rand::thread_rng());

                if let Some(line) = available_lines {
                    self.lanes[3].add_car(Car::new(Direction::North, line));
                }
            }
        }
    }

    pub fn add_car_random(&mut self) {
        match gen_range(0, 3) {
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
