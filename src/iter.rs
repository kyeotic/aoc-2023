use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn intersection<T: Eq + Hash + Copy>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let unique_a = a.iter().copied().collect::<HashSet<_>>();
    let unique_b = b.iter().copied().collect::<HashSet<_>>();

    unique_a.intersection(&unique_b).copied().collect()
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn grid_iterator<T: Copy + Debug>(grid: &Vec<Vec<T>>) -> GridIter<'_, T> {
    GridIter {
        grid,
        x_max: grid.len() - 1,
        y_max: grid.first().unwrap().len() - 1,
        cursor: Some((0, 0)),
    }
}

pub struct GridIter<'a, T: Copy + Debug> {
    grid: &'a Vec<Vec<T>>,
    x_max: usize,
    y_max: usize,
    cursor: Option<(usize, usize)>,
}

impl<'a, T: Copy + Debug> Iterator for GridIter<'a, T> {
    type Item = (T, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.cursor?;
        let n = self.grid[x][y];

        if x == self.x_max && y == self.y_max {
            self.cursor = None
        }
        self.cursor = Some(if x == self.x_max {
            (0, y + 1)
        } else {
            (x + 1, y)
        });

        Some((n, x, y))
    }
}
