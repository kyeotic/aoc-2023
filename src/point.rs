use itertools::Itertools;

lazy_static! {
    static ref ADJACENT_DIAGONAL: Vec<(i32, i32)> =
        (-1..=1).cartesian_product(-1..=1).collect_vec();
    static ref ADJACENT_CARDINAL: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new_u(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
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

    pub fn to(&self, mx: i32, my: i32) -> Self {
        Self::new(self.x + mx, self.y + my)
    }
}

fn map_from(p: &Point, to: Vec<(i32, i32)>) -> Vec<Point> {
    to.iter().map(|(dx, dy)| p.to(*dx, *dy)).collect_vec()
}
