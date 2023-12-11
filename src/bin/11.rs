#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use anyhow::{Context, Result};
use aoc::{
    get_input,
    grid::{into_columns, Grid, GridPoint},
    math::is_between,
    point::Point,
    report,
};
use itertools::Itertools;

fn main() {
    let input = get_input("11");

    report(
        (|| part1(&input), Some(374), Some(10313550)),
        (|| part2(&input), None, Some(611998089572)),
    );
}

fn part1(input: &str) -> u64 {
    sum_distance(input, 2)
}

fn part2(input: &str) -> u64 {
    sum_distance(input, 1000000)
}

fn sum_distance(input: &str, factor: u64) -> u64 {
    let image: Grid<Pixel> = Grid::from(&input);
    let (extra_rows, extra_cols) = get_expansion(&image);

    image
        .iter()
        .filter(|g| g.value == Pixel::Galaxy)
        .combinations(2)
        .map(|pair| {
            let a = pair.first().unwrap();
            let b = pair.last().unwrap();
            Point::from(a).manhattan_distance(&b.into()) as u64
                + ((factor - 1)
                    * (count_empty(&extra_rows, a.y, b.y) + count_empty(&extra_cols, a.x, b.x)))
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Space,
    Galaxy,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '.' => Pixel::Space,
            '#' => Pixel::Galaxy,
            _ => panic!("invalid pixel"),
        }
    }
}

fn get_expansion(grid: &Grid<Pixel>) -> (Vec<u64>, Vec<u64>) {
    let rows = filter_empty(&grid.points);
    let cols = filter_empty(&into_columns(&grid.points));

    (rows, cols)
}

fn filter_empty(grid: &[Vec<GridPoint<Pixel>>]) -> Vec<u64> {
    grid.iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if is_line_empty(&line) {
                Some(i as u64)
            } else {
                None
            }
        })
        .collect_vec()
}

fn is_line_empty(line: &[GridPoint<Pixel>]) -> bool {
    line.iter().all(|p| &p.value == &Pixel::Space)
}

fn count_empty(lines: &[u64], a: i64, b: i64) -> u64 {
    lines
        .iter()
        .filter(|i| is_between(**i as i64, a, b))
        .count() as u64
}
