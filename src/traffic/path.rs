use crate::traffic::car::Turning;
use crate::traffic::Direction;

/*
pub enum SectorStatus {
    Taken,
    Free
}
*/

#[derive(Clone, Debug)]
pub struct Sector {
    x: usize,
    y: usize,
    //    status: SectorStatus
}

impl Sector {
    pub fn new(x: usize, y: usize) -> Sector {
        Sector {
            x,
            y,
            //status: SectorStatus::Free
        }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}
#[derive(Clone)]
pub struct Path {
    pub current: Sector,
    pub sectors: Vec<Sector>,
}

impl Path {
    pub fn new(direction: &Direction, turning: &Turning) -> Path {
        let sectors = match turning {
            Turning::Left => left_turn(direction),
            Turning::Straight => go_straight(direction),
            Turning::Right => right_turn(direction),
        };
        Path {
            current: sectors[0].clone(),
            sectors,
        }
    }
}

/*
 *       x x x x x x
 *       0 1 2 3 4 5
 *  y 0 |_|_|_|_|_|_|
 *  y 1 |_|_|_|_|_|_|
 *  y 2 |_|_|_|_|_|_|
 *  y 3 |_|_|_|_|_|_|
 *  y 4 |_|_|_|_|_|_|
 *  y 5 |_|_|_|_|_|_|
 *
 *
 * PATHS FOR NORTH:
 * RIGHT: (x0, y0) -> (x0,y0)
 * STRAIGHT: (x1, y0) -> (x1, y5)
 * LEFT: (x2, y0) -> (x2, y3) -> (x5, y3)
 *
 * PATHS FOR EAST:
 * RIGHT: (x5, y0) -> (x5, y0)
 * STRAIGHT: (x5, y1) -> (x0, y1)
 * LEFT: (x5, y2) -> (x2, y2) -> (x2, y5)
 *
 * PATHS FOR SOUTH:
 * RIGHT: (x5, y5) -> (x5, y5)
 * STRAIGHT: (x4, y5) -> (x4, y0)
 * LEFT: (x3, y5) -> (x3, y2) -> (x0, y2)
 *
 * PATHS FOR WEST:
 * RIGHT: (x0, y5) -> (x0, y5)
 * STRAIGHT: (x0, y4) -> (x5, y4)
 * LEFT: (x0, y3) -> (x3, y3) -> (x3, y0)
 *
 */

fn left_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => vec![
            Sector::new(5, 3), // Exit
            Sector::new(2, 3), // Turning-point
            Sector::new(2, 0), // Entry
        ],
        Direction::East => vec![
            Sector::new(5, 2), // Entry
            Sector::new(2, 2), // Turning-point
            Sector::new(2, 5), // Exit
        ],
        Direction::South => vec![
            Sector::new(3, 6), // Entry
            Sector::new(3, 2), // Turning-point
            Sector::new(0, 2), // Exit
        ],
        Direction::West => vec![
            Sector::new(0, 3), // Entry
            Sector::new(3, 3), // Turning-point
            Sector::new(3, 0), // Exit
        ],
    }
}

fn go_straight(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => vec![
            Sector::new(1, 5), // Exit
            Sector::new(1, 0), // Entry
        ],
        Direction::East => vec![
            Sector::new(0, 1), // Exit
            Sector::new(5, 1), // Entry
        ],
        Direction::South => vec![
            Sector::new(4, 0), // Exit
            Sector::new(4, 5), // Entry
        ],
        Direction::West => vec![
            Sector::new(5, 4), // Exit
            Sector::new(0, 4), // Entry
        ],
    }
}

fn right_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => vec![Sector::new(0, 0)],
        Direction::East => vec![Sector::new(5, 0)],
        Direction::South => vec![Sector::new(5, 5)],
        Direction::West => vec![Sector::new(0, 5)],
    }
}
