use crate::traffic::Direction;
use crate::traffic::state::Turning;

pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x, y
        }
    }
}
pub struct Path {
    current: usize,
    points: Vec<Point>
}

impl Path {
    pub fn new(direction: Direction, turning: Turning) -> Path {
        Path {
            current: 0,
            points: match turning {
                Turning::Left => left_turn(direction),
                Turning::Straight => go_straight(direction),
                Turning::Right => right_turn(direction),
            }
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

fn left_turn(direction: Direction) -> Vec<Point> {
    match direction {
        Direction::North => vec![
            Point::new(2, 0), // Entry
            Point::new(2, 3), // Turning-point
            Point::new(5, 3), // Exit
        ],
        Direction::East => vec![
            Point::new(5, 2),
            Point::new(2, 2),
            Point::new(2, 5),
        ],
        Direction::South => vec![
            Point::new(3, 6),
            Point::new(3, 2),
            Point::new(0, 2),
        ],
        Direction::West => vec![
            Point::new(0, 3),
            Point::new(3, 3),
            Point::new(3, 0),
        ]
    }
}

fn go_straight(direction: Direction) -> Vec<Point> {
    match direction {
        Direction::North => vec![Point::new(1, 0), Point::new(1, 5)],
        Direction::East => vec![Point::new(5, 1), Point::new(0, 1)],
        Direction::South => vec![Point::new(4, 5), Point::new(4, 0)],
        Direction::West => vec![Point::new(0, 4), Point::new(5, 4)]
    }
}

fn right_turn(direction: Direction) -> Vec<Point> {
    match direction {
        Direction::North => vec![Point::new(0, 0)],
        Direction::East => vec![Point::new(5, 0)],
        Direction::South => vec![Point::new(5, 5)],
        Direction::West => vec![Point::new(0, 5)]
    }
}

