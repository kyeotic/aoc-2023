#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use anyhow::{Context, Result};
use aoc::{
    get_input,
    grid::{Grid, GridPoint},
    report,
};
use indexmap::IndexMap;
use itertools::Itertools;

fn main() {
    let input = get_input("17");

    report(
        (|| part1(&input), Some(102), None),
        (|| part2(&input), None, None),
    );
}

fn part1(input: &str) -> u64 {
    let grid: Grid<u32> = Grid::from_num(input);
    let path: IndexMap<GridPoint<u32>, u32> = IndexMap::new();

    println!("{grid:?}");
    1
}

fn part2(_input: &str) -> u64 {
    2
}
