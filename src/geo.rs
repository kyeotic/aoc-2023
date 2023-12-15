use itertools::Itertools;

type Grid<T> = Vec<Vec<T>>;

pub fn transpose<T>(grid: &Grid<T>) -> Grid<T>
where
    T: Clone,
{
    (0..grid[0].len())
        .map(|i| grid.iter().map(|inner| inner[i].clone()).collect_vec())
        .collect()
}

pub fn row_reverse<T>(grid: &Grid<T>) -> Grid<T>
where
    T: Clone,
{
    grid.iter()
        .map(|row| row.iter().rev().cloned().collect_vec())
        .collect_vec()
}

pub fn rotate_right<T>(grid: &Grid<T>) -> Grid<T>
where
    T: Clone,
{
    row_reverse(&transpose(&grid))
}

pub fn rotate_left<T>(grid: &Grid<T>) -> Grid<T>
where
    T: Clone,
{
    transpose(&row_reverse(&grid))
}

pub fn rotate_180<T>(grid: &Grid<T>) -> Grid<T>
where
    T: Clone,
{
    // Rotate by +180:
    // Method 1: Rotate by +90 twice
    // Method 2: Reverse each row and then reverse each column
    rotate_left(&rotate_left(&grid))
}
