use std::fmt::{Display, Formatter};

use crate::traffic::car::Car;
use crate::traffic::path::Sector;
use crate::traffic::Turning;

type Sectors = [[Option<Car>; 12]; 12];
type Intersection = [Vec<Option<Car>>; 6];

#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
    pub sectors: Sectors, // None if no car, Some if there is a car
    occupied_sectors: Vec<Sector>,
    empty_sectors: Vec<Sector>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            sectors: generate_empty_grid(),
            occupied_sectors: Vec::new(),
            empty_sectors: Vec::new(),
        }
    }

    pub fn update_grid(&mut self, car: Car) {
        if car.turning == Turning::Right {
            return;
        }
        let x = car.get_sector().get_x();
        let y = car.get_sector().get_y();
        self.sectors[x][y] = Some(car)

        /*
        for (x, column) in self.sectors.clone().into_iter().enumerate() {
            for (y, car) in column.into_iter().enumerate() {
                if car.is_none() {
                    self.empty_sectors.push(Sector::new(x, y));
                } else {
                    self.occupied_sectors.push(Sector::new(x, y));
                }
            }
        }

         */
    }

    pub fn get_car_at_coords(&self, x: usize, y: usize) -> Option<Car> {
        self.sectors[x][y].clone()
    }

    pub fn get_intersection(&self) -> Intersection {
        let mut intersection = generate_intersection();
        for (i, r) in self.sectors.iter().skip(3).take(6).enumerate() {
            let mut row = Vec::new();
            r.clone()
                .into_iter()
                .skip(3)
                .take(6)
                .for_each(|c| row.push(c));
            intersection[i] = row;
        }
        intersection
    }

    pub fn get_occupied(&self) -> &Vec<Sector> {
        &self.occupied_sectors
    }
    pub fn get_empty(&self) -> &Vec<Sector> {
        &self.empty_sectors
    }

    pub fn display_intersection(&self) {
        for (i, col) in self.get_intersection().iter().enumerate() {
            print!("{} ", i + 1);
            col.iter().for_each(|car| {
                if car.is_none() {
                    print!("[ ]");
                } else {
                    print!("[X]")
                }
            });
            println!();
        }
        println!("--------------------");
    }

    pub fn refresh_grid(&mut self) {
        self.sectors = generate_empty_grid();
    }
}

fn generate_intersection() -> Intersection {
    [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ]
}
fn generate_empty_grid() -> Sectors {
    [
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
        [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ],
    ]
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
            self.sectors[0]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[1]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[2]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[3]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[4]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[5]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[6]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[7]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[8]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[9]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[10]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>(),
            self.sectors[11]
                .iter()
                .map(|c| c.is_some())
                .collect::<Vec<bool>>()
        )
    }
}
impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
