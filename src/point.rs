use itertools::Itertools;

lazy_static! {
    static ref ADJACENT_DIAGONAL: Vec<(i64, i64)> =
        (-1..=1).cartesian_product(-1..=1).collect_vec();
    static ref ADJACENT_CARDINAL: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn new_u(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }

    pub fn is_adjacent(&self, p: &Point) -> bool {
        self.get_adjacent().iter().any(|px| px == p)
    }

    pub fn get_adjacent(&self) -> Vec<Point> {
        map_from(self, ADJACENT_CARDINAL.to_vec())
    }

    pub fn is_adjacent_diagonal(&self, p: &Point) -> bool {
        self.get_adjacent_diagonal().iter().any(|px| px == p)
    }

    pub fn get_adjacent_diagonal(&self) -> Vec<Point> {
        map_from(self, ADJACENT_DIAGONAL.to_vec())
    }

    pub fn to(&self, mx: i64, my: i64) -> Self {
        Self::new(self.x + mx, self.y + my)
    }

    pub fn distance(&self, to: &Point) -> (i64, i64) {
        (to.x - self.x, to.y - self.y)
    }
}

fn map_from(p: &Point, to: Vec<(i64, i64)>) -> Vec<Point> {
    to.iter().map(|(dx, dy)| p.to(*dx, *dy)).collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub fn inverse(&self) -> CardinalDirection {
        match self {
            CardinalDirection::Up => CardinalDirection::Down,
            CardinalDirection::Down => CardinalDirection::Up,
            CardinalDirection::Left => CardinalDirection::Right,
            CardinalDirection::Right => CardinalDirection::Left,
        }
    }
}

// impl From<&(i64, i64)> for CardinalDirection {
//     fn from(d: &(i64, i64)) -> Self {
//         match d {
//             (0, -1) => Self::Down,
//             (0, 1) => Self::Up,
//             (-1, 0) => Self::Right,
//             (1, 0) => Self::Left,
//             _ => panic!("Invalid Cardinal {d:?}"),
//         }
//     }
// }
