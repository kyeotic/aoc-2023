use itertools::Itertools;

use crate::point::Point;

// use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    pub points: Vec<Vec<GridPoint<T>>>,
}

impl<T: Copy> Grid<T> {
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

    pub fn get(&self, x: usize, y: usize) -> Option<GridPoint<T>> {
        Some(self.points.get(y as usize)?.get(x as usize)?.to_owned())
    }

    //     fn is_inside(&self, p: &Point) -> bool {
    //         let x_max = self.0.first().unwrap().len();
    //         let y_max = self.0.len();
    //         p.x >= 0 && p.x < x_max as i32 && p.y >= 0 && p.y < y_max as i32
    //     }

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            grid: &self,
            cursor: Some((0, 0)), // to_visit: (0..self.points.len())
                                  //         .cartesian_product(0..self.points[0].len())
                                  //         .collect(),
        }
    }

    // fn rows(&self) -> Vec<GridPoint<T>> {
    //     self.points.
    // }
}

pub struct GridIter<'a, T: Copy> {
    grid: &'a Grid<T>,
    cursor: Option<(usize, usize)>,
}

impl<'a, T: Copy> Iterator for GridIter<'a, T> {
    type Item = GridPoint<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let x_max = self.grid.points.first()?.len() - 1;
        let y_max = self.grid.points.len() - 1;

        let (x, y) = self.cursor?;
        let n = Some(self.grid.get(x, y)?);

        if x == x_max && y == y_max {
            self.cursor = None
        }
        self.cursor = Some(if x == x_max { (0, y + 1) } else { (x + 1, y) });

        n
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GridPoint<T> {
    pub x: i32,
    pub y: i32,
    pub value: T,
}

impl<T: Clone> GridPoint<T> {
    pub fn new(x: i32, y: i32, value: T) -> Self {
        Self { x, y, value }
    }

    pub fn new_u(x: usize, y: usize, value: T) -> Self {
        Self::new(x as i32, y as i32, value)
    }

    pub fn point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
