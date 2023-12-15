use crate::point::{CardinalDirection, Point};
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    pub points: Vec<Vec<GridPoint<T>>>,
}

impl<T> Grid<T>
where
    T: Copy + From<char> + Debug,
{
    pub fn from(s: &str) -> Self {
        Self::new(
            s.lines()
                .map(|l| l.chars().map(|c| c.into()).collect_vec())
                .collect_vec(),
        )
    }
}

impl<T> Grid<T>
where
    T: Copy + Debug,
{
    pub fn new(vals: Vec<Vec<T>>) -> Self {
        Self {
            points: vals
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    let y = y.clone();
                    row.iter()
                        .enumerate()
                        .map(|(x, val)| GridPoint::new_u(x, y, *val))
                        .collect_vec()
                })
                .collect(),
        }
    }

    pub fn get(&self, x: i64, y: i64) -> Option<GridPoint<T>> {
        self.get_u(x as usize, y as usize)
    }

    pub fn get_u(&self, x: usize, y: usize) -> Option<GridPoint<T>> {
        Some(self.points.get(y)?.get(x)?.to_owned())
    }

    pub fn get_point(&self, p: &Point) -> Option<GridPoint<T>> {
        Some(self.points.get(p.y as usize)?.get(p.x as usize)?.to_owned())
    }

    pub fn get_neighbors(&self, p: &GridPoint<T>) -> Vec<GridPoint<T>> {
        self.to_grid(&Point::from(p).get_adjacent())
    }

    pub fn get_neighbor_steps(&self, p: &GridPoint<T>) -> Vec<(GridPoint<T>, CardinalDirection)> {
        let p = Point::from(p);
        let neighbors = self.to_grid(&p.get_adjacent());
        neighbors
            .into_iter()
            .map(|n| {
                (
                    n,
                    match &p.distance(&n.into()) {
                        (0, -1) => CardinalDirection::Up,
                        (0, 1) => CardinalDirection::Down,
                        (-1, 0) => CardinalDirection::Left,
                        (1, 0) => CardinalDirection::Right,
                        _ => panic!("Invalid distance {:?}", p.distance(&n.into())),
                    },
                )
            })
            .collect_vec()
    }

    pub fn to_grid(&self, points: &Vec<Point>) -> Vec<GridPoint<T>> {
        points
            .iter()
            .filter_map(|p| {
                if self.is_inside(p) {
                    self.get_point(p)
                } else {
                    None
                }
            })
            .collect_vec()
    }

    pub fn step(&self, from: &GridPoint<T>, dir: CardinalDirection) -> Option<GridPoint<T>> {
        match dir {
            CardinalDirection::Up => self.get(from.x, from.y - 1),
            CardinalDirection::Down => self.get(from.x, from.y + 1),
            CardinalDirection::Left => self.get(from.x - 1, from.y),
            CardinalDirection::Right => self.get(from.x + 1, from.y),
        }
    }

    pub fn is_inside(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.size_x() && p.y >= 0 && p.y < self.size_y()
    }

    pub fn size_x(&self) -> i64 {
        (self.points.first().unwrap().len() - 1) as i64
    }

    pub fn size_y(&self) -> i64 {
        (self.points.len() - 1) as i64
    }

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            grid: &self,
            x_max: self.size_x() as usize,
            y_max: self.size_y() as usize,
            cursor: Some((0, 0)),
        }
    }

    // fn rows(&self) -> Vec<GridPoint<T>> {
    //     self.points.
    // }
}

pub struct GridIter<'a, T: Copy + Debug> {
    grid: &'a Grid<T>,
    x_max: usize,
    y_max: usize,
    cursor: Option<(usize, usize)>,
}

impl<'a, T: Copy + Debug> Iterator for GridIter<'a, T> {
    type Item = GridPoint<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.cursor?;
        let n = Some(self.grid.get_u(x, y)?);

        if x == self.x_max && y == self.y_max {
            self.cursor = None
        }
        self.cursor = Some(if x == self.x_max {
            (0, y + 1)
        } else {
            (x + 1, y)
        });

        n
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct GridPoint<T> {
    pub x: i64,
    pub y: i64,
    pub value: T,
}

impl<T: Clone> GridPoint<T> {
    pub fn new(x: i64, y: i64, value: T) -> Self {
        Self { x, y, value }
    }

    pub fn new_u(x: usize, y: usize, value: T) -> Self {
        Self::new(x as i64, y as i64, value)
    }
}

impl<T> From<GridPoint<T>> for Point {
    fn from(item: GridPoint<T>) -> Self {
        Point {
            x: item.x,
            y: item.y,
        }
    }
}

impl<T> From<&GridPoint<T>> for Point {
    fn from(item: &GridPoint<T>) -> Self {
        Point {
            x: item.x,
            y: item.y,
        }
    }
}

pub fn into_columns<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    grid.first()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, _)| grid.iter().map(|row| row[i].clone()).collect_vec())
        .collect_vec()
}
