#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use anyhow::{Context, Result};
use aoc::{
    get_input,
    grid::into_columns,
    iter::grid_iterator,
    parse::{as_char_grid, as_paragraphs, char_grid_to_string},
    report,
};
use itertools::Itertools;

fn main() {
    let input = get_input("13");

    report(
        (|| part1(&input), Some(405), Some(34202)),
        (|| part2(&input), Some(400), Some(34230)),
    );
}

fn part1(input: &str) -> usize {
    as_paragraphs(input)
        .into_iter()
        .map(as_char_grid)
        .map(|g| grid_value(&g, None).unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    as_paragraphs(input)
        .into_iter()
        .map(as_char_grid)
        .map(|g| {
            let val = grid_value(&g, None).unwrap();

            grid_iterator(&g)
                .find_map(|(_, x, y)| grid_value(&smudge(&g, x, y), Some(val)))
                .unwrap()
        })
        .sum()
}

fn grid_value(grid: &Vec<Vec<char>>, skip: Option<usize>) -> Option<usize> {
    find_reflection(&grid, skip)
        .or_else(|| find_reflection(&into_columns(&grid), skip.map(|s| s / 100)).map(|x| x * 100))
}

fn find_reflection(grid: &Vec<Vec<char>>, skip: Option<usize>) -> Option<usize> {
    let len = grid.first()?.len();
    (1..len).find(|i| {
        grid.iter().all(|r| {
            r[0..*i]
                .iter()
                .rev()
                .take(len - i)
                .eq(r[*i..].iter().take(*i))
                && Some(*i) != skip
        })
    })
}

fn smudge(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Vec<char>> {
    let s = match grid[x][y] {
        '.' => '#',
        '#' => '.',
        _ => unreachable!("invalid smudge"),
    };

    let mut grid = grid.clone();
    grid[x][y] = s;
    grid
}
