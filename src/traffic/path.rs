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
#[derive(Clone, Debug)]
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
            Sector::new(5, 0),  // Entry
            Sector::new(5, 6),  // Turning-point
            Sector::new(11, 6), // Exit
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 5), // Entry
            Sector::new(5, 5),  // Turning-point
            Sector::new(5, 11), // Exit
        ]),
        Direction::South => get_path(vec![
            Sector::new(6, 11), // Entry
            Sector::new(6, 5),  // Turning-point
            Sector::new(0, 5),  // Exit
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 6), // Entry
            Sector::new(6, 6), // Turning-point
            Sector::new(6, 0), // Exit
        ]),
    }
}

fn go_straight(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(4, 0),  // Entry
            Sector::new(4, 5),  // Mid-point
            Sector::new(4, 11), // Exit
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 4), // Entry
            Sector::new(5, 4),  // Mid-point
            Sector::new(0, 4),  // Exit
        ]),
        Direction::South => get_path(vec![
            Sector::new(7, 11), // Entry
            Sector::new(7, 5),  // Mid-point
            Sector::new(7, 0),  // Exit
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 7),  // Entry
            Sector::new(5, 7),  // Mid-point
            Sector::new(11, 7), // Exit
        ]),
    }
}

fn right_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(3, 0), // Entry
            Sector::new(3, 3), // Turning point
            Sector::new(0, 3), // Exit
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 3), // Entry
            Sector::new(8, 3),  // Turning point
            Sector::new(8, 0),  // Exit
        ]),
        Direction::South => get_path(vec![
            Sector::new(8, 11), // Entry
            Sector::new(8, 8),  // Turning point
            Sector::new(11, 8), // Exit
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 8),  // Entry
            Sector::new(3, 8),  // Turning point
            Sector::new(3, 11), // Exit
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
        path.push(Sector::new(x, y));
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
        path.push(Sector::new(x, y));
    }
    path
}
