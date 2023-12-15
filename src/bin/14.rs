#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

use anyhow::{Context, Result};
use aoc::{
    geo::{rotate_180, rotate_left, rotate_right},
    get_input,
    parse::{as_char_grid, char_grid_to_string},
    report,
};
use indexmap::IndexMap;
use itertools::Itertools;

type Grid = Vec<Vec<char>>;

fn main() {
    let input = get_input("14");

    report(
        (|| part1(&input), Some(136), Some(112773)),
        (|| part2(&input), Some(64), Some(98894)),
    );
}

fn part1(input: &str) -> u64 {
    calc_load(&tilt(&rotate_left(&as_char_grid(input))))
}

fn part2(input: &str) -> u64 {
    let mut cache: IndexMap<Grid, u64> = IndexMap::new();
    let mut grid = rotate_180(&as_char_grid(input));
    let cycles = 1_000_000_000;

    for i in 0..cycles {
        for _ in 0..4 {
            grid = tilt(&rotate_right(&grid));
        }

        let key = grid.clone();

        if let Some(start) = cache.get(&key) {
            let cycle_len = i - start;
            let cycles_ran = (cycles - start) / cycle_len;
            let cycle_index = cycles - ((cycle_len * cycles_ran) + start);
            let grid_index = start - 1 + cycle_index;

            grid = cache.keys().nth(grid_index as usize).unwrap().clone();
            break;
        }

        cache.insert(key, i);
    }

    calc_load(&rotate_right(&grid))
}

// Its easier to work in rows than columns, so this tilts LEFT
// Make sure to rotate correctly first
fn tilt(grid: &Grid) -> Grid {
    grid.iter()
        .map(|row| {
            row.iter()
                .collect::<String>()
                .split("#")
                .map(|s| s.chars().sorted().rev().collect::<String>())
                .join("#")
                .chars()
                .collect_vec()
        })
        .collect_vec()
}

// Since tilt works LEFT this calculates by ROW, not COLUMN
// so that another rotation after tilt is not required
fn calc_load(grid: &Grid) -> u64 {
    grid.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, c)| if c == &'O' { row.len() - i } else { 0 })
                .sum::<usize>() as u64
        })
        .sum()
}
