use crate::traffic::car::Car;
use crate::traffic::grid::Grid;
use crate::traffic::lane::Lane;
use crate::traffic::statistics::*;
use macroquad::rand::gen_range;

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
                Lane::new(Direction::North),
                Lane::new(Direction::East),
                Lane::new(Direction::South),
                Lane::new(Direction::West),
            ],
            grid: Grid::new(),
            stats: Statistics::new(),
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
            Direction::North => self.lanes[0].add_car(Car::new(Direction::North)),
            Direction::East => self.lanes[1].add_car(Car::new(Direction::East)),
            Direction::South => self.lanes[2].add_car(Car::new(Direction::South)),
            Direction::West => self.lanes[3].add_car(Car::new(Direction::West)),
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
