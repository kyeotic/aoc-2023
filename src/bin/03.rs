#![allow(dead_code, unused_imports)]
use std::ops::{self, Range};

use aoc::grid::{Grid, GridPoint};
use aoc::point::Point;
use aoc::{get_input, report};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = get_input("03");
    let manual = Manual::from(&input);

    report(
        (|| part1(&manual), Some(4361), Some(533775)),
        (|| part2(&manual), Some(467835), Some(78236071)),
    );
}

fn part1(manual: &Manual) -> u64 {
    manual
        .numbers
        .iter()
        .filter(|pn| {
            pn.points.iter().any(|p| {
                manual
                    .symbols
                    .iter()
                    .any(|s| p.is_adjacent_diagonal(&(s.into())))
            })
        })
        .map(|p| p.value)
        .sum()
}

fn part2(manual: &Manual) -> u64 {
    manual
        .get_gears()
        .iter()
        .filter_map(|g| {
            let adj_parts = manual
                .numbers
                .iter()
                .filter(|pn| pn.points.iter().any(|p| p.is_adjacent_diagonal(&g.into())))
                .map(|p| p.value)
                .collect_vec();

            if adj_parts.len() == 2 {
                Some(adj_parts.iter().product::<u64>())
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Manual {
    grid: Grid<char>,
    numbers: Vec<PartNumber>,
    symbols: Vec<GridPoint<char>>,
}

impl Manual {
    fn from(input: &str) -> Self {
        let grid = Grid::from(input);
        Self {
            numbers: get_parts(&grid),
            symbols: get_symbols(&grid),
            grid,
        }
    }

    fn get_gears(&self) -> Vec<GridPoint<char>> {
        self.symbols
            .iter()
            .filter(|s| s.value == '*')
            .copied()
            .collect()
    }
}

fn get_parts(grid: &Grid<char>) -> Vec<PartNumber> {
    let num = Regex::new(r"(\d+)").unwrap();

    grid.points
        .iter()
        .enumerate()
        .map(|(y, row)| {
            num.captures_iter(&row.iter().map(|g| g.value).collect::<String>())
                .map(|c| {
                    let m = c.get(0).unwrap();
                    PartNumber {
                        value: m.as_str().parse().unwrap(),
                        points: m.range().map(|x| Point::new_u(x, y)).collect(),
                    }
                })
                .collect_vec()
        })
        .flatten()
        .collect()
}

fn get_symbols(grid: &Grid<char>) -> Vec<GridPoint<char>> {
    grid.iter()
        .filter_map(|p| if is_symbol(p.value) { Some(p) } else { None })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct PartNumber {
    value: u64,
    points: Vec<Point>,
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn is_empty(c: char) -> bool {
    c == '.'
}
