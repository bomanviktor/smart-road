use std::fmt::{Display, Formatter};

use crate::traffic::car::Turning;
use crate::traffic::{Direction, Moving};

/*
pub enum SectorStatus {
    Taken,
    Free
}
*/

#[derive(Eq, Clone, Debug)]
pub struct Sector {
    x: usize,
    y: usize,
    pub moving: Moving, //    status: SectorStatus
}

impl PartialEq for Sector {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.y && self.moving != other.moving
    }
}

impl Sector {
    pub fn new(x: usize, y: usize, moving: Moving) -> Sector {
        Sector {
            x,
            y,
            moving, //status: SectorStatus::Free
        }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Path {
    pub current: usize,
    pub sectors: Vec<Sector>,
}

impl Path {
    pub fn new(direction: &Direction, turning: &Turning) -> Path {
        Path {
            current: 0,
            sectors: match turning {
                Turning::Left => left_turn(direction),
                Turning::Straight => go_straight(direction),
                Turning::Right => right_turn(direction),
            },
        }
    }
}

/*
 *                  x
 *       0 1 2 3 4 5 6 7 8 9 10 11
 *    0       |_|_|_|_|_|_|        0
 *    1       |_|_|_|_|_|_|        1
 *    2       |_|_|_|_|_|_|        2
 *    3 |_|_|_|_|_|_|_|_|_|_|_|_|  3
 *    4 |_|_|_|_|_|_|_|_|_|_|_|_|  4
 *  y 5 |_|_|_|_|_|_|_|_|_|_|_|_|  5 y
 *    6 |_|_|_|_|_|_|_|_|_|_|_|_|  6
 *    7 |_|_|_|_|_|_|_|_|_|_|_|_|  7
 *    8 |_|_|_|_|_|_|_|_|_|_|_|_|  8
 *    9       |_|_|_|_|_|_|        9
 *    10      |_|_|_|_|_|_|        10
 *    11      |_|_|_|_|_|_|        11
 *       0 1 2 3 4 5 6 7 8 9 10 11
 *                  x
 *
 * PATHS FOR NORTH:
 * LEFT: (x5, y0) -> (x5, y6) -> (x11, y6)
 * STRAIGHT: (x4, y0) -> (x4, y11)
 * RIGHT: (x3, y0) -> (x3, y3) -> (x0, y3)
 *
 * PATHS FOR EAST:
 * LEFT: (x11, y5) -> (x5, y5) -> (x5, y11)
 * STRAIGHT: (x11, y4) -> (x0, y4)
 * RIGHT: (x11, y3) -> (x8, y3) -> (x8, y0)
 *
 * PATHS FOR SOUTH:
 * LEFT: (x6, y11) -> (x6, y5) -> (x0, y5)
 * STRAIGHT: (x7, y11) -> (x7, y0)
 * RIGHT: (x8, y11) -> (x8, y8) -> (x11, y8)
 *
 * PATHS FOR WEST:
 * LEFT: (x0, y6) -> (x6, y6) -> (x6, y0)
 * STRAIGHT: (x0, y7) -> (x11, y7)
 * RIGHT: (x0, y8) -> (x3, y8) -> (x3, y11)
 *
 */

fn left_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(5, 0, Moving::Down),   // Entry
            Sector::new(5, 6, Moving::Right),  // Turning-point
            Sector::new(11, 6, Moving::Right), // Exit
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 5, Moving::Left), // Entry
            Sector::new(5, 5, Moving::Down),  // Turning-point
            Sector::new(5, 11, Moving::Down), // Exit
        ]),
        Direction::South => get_path(vec![
            Sector::new(6, 11, Moving::Up),  // Entry
            Sector::new(6, 5, Moving::Left), // Turning-point
            Sector::new(0, 5, Moving::Left), // Exit
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 6, Moving::Right), // Entry
            Sector::new(6, 6, Moving::Up),    // Turning-point
            Sector::new(6, 0, Moving::Up),    // Exit
        ]),
    }
}

fn go_straight(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(4, 0, Moving::Down),  // Entry
            Sector::new(4, 5, Moving::Down),  // Mid-point
            Sector::new(4, 11, Moving::Down), // Exit
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 4, Moving::Left), // Entry
            Sector::new(5, 4, Moving::Left),  // Mid-point
            Sector::new(0, 4, Moving::Left),  // Exit
        ]),
        Direction::South => get_path(vec![
            Sector::new(7, 11, Moving::Up), // Entry
            Sector::new(7, 5, Moving::Up),  // Mid-point
            Sector::new(7, 0, Moving::Up),  // Exit
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 7, Moving::Right),  // Entry
            Sector::new(5, 7, Moving::Right),  // Mid-point
            Sector::new(11, 7, Moving::Right), // Exit
        ]),
    }
}

fn right_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(3, 0, Moving::Down), // Entry
            Sector::new(3, 3, Moving::Left), // Turning point
            Sector::new(0, 3, Moving::Left), // Exit
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 3, Moving::Left), // Entry
            Sector::new(8, 3, Moving::Down),  // Turning point
            Sector::new(8, 0, Moving::Down),  // Exit
        ]),
        Direction::South => get_path(vec![
            Sector::new(8, 11, Moving::Up),    // Entry
            Sector::new(8, 8, Moving::Right),  // Turning point
            Sector::new(11, 8, Moving::Right), // Exit
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 8, Moving::Right), // Entry
            Sector::new(3, 8, Moving::Up),    // Turning point
            Sector::new(3, 11, Moving::Up),   // Exit
        ]),
    }
}

// Helper function to get all sectors in the path
fn get_path(sectors: Vec<Sector>) -> Vec<Sector> {
    let mut path = vec![sectors[0].clone()];
    let mut x: usize = sectors[0].x;
    let mut y: usize = sectors[0].y;
    while x != sectors[1].x || y != sectors[1].y {
        if x < sectors[1].x {
            x += 1;
        }
        if x > sectors[1].x {
            x -= 1;
        }
        if y < sectors[1].y {
            y += 1;
        }
        if y > sectors[1].y {
            y -= 1;
        }

        path.push(Sector::new(x, y, sectors[0].clone().moving));
    }
    while x != sectors[2].x || y != sectors[2].y {
        if x < sectors[2].x {
            x += 1;
        }
        if x > sectors[2].x {
            x -= 1;
        }
        if y < sectors[2].y {
            y += 1;
        }
        if y > sectors[2].y {
            y -= 1;
        }
        path.push(Sector::new(x, y, sectors[1].clone().moving));
    }
    path
}

impl Display for Sector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sector: (x: {}, y: {})", self.x, self.y)
    }
}
