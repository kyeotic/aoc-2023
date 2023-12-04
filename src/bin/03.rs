#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::ops::Range;

use aoc::{get_input, report};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = get_input("03");
    let manual = Manual::from(&input);

    let a = part1(&manual);
    let b = part2(&manual);

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    assert_eq!(a, 533775);
    assert_eq!(b, 78236071);
}

fn part1(manual: &Manual) -> u32 {
    let parts = manual.get_parts();

    parts
        .iter()
        .filter(|p| {
            p.range
                .clone()
                // Get adjacent to all points in number
                .map(|x| manual.get_adjacent(x, p.row))
                .flatten()
                .unique()
                .any(is_symbol)
        })
        .map(|p| p.value)
        .sum()
}

fn part2(manual: &Manual) -> u32 {
    let parts = manual.get_parts();

    manual
        .get_gears()
        .iter()
        .filter_map(|g| {
            let adj = manual.get_adjacent_points(g.x, g.y);
            let adj_parts = parts
                .iter()
                .filter(|p| adj.iter().any(|(x, y)| p.row == *y && p.range.contains(&x)))
                .map(|p| p.value)
                .collect_vec();

            if adj_parts.len() == 2 {
                Some(adj_parts.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

lazy_static! {
    static ref ADJACENT: Vec<(i32, i32)> = (-1..=1).cartesian_product(-1..=1).collect_vec();
}

#[derive(Debug, Clone)]
struct Manual(Vec<Vec<char>>);

impl Manual {
    fn from(input: &str) -> Self {
        Self(input.lines().map(|l| l.chars().collect_vec()).collect())
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.0.get(y).unwrap().get(x).unwrap().to_owned()
    }

    fn get_parts(&self) -> Vec<PartNumber> {
        let num = Regex::new(r"(\d+)").unwrap();

        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                num.captures_iter(&row.iter().collect::<String>())
                    .map(|c| {
                        let m = c.get(0).unwrap();
                        PartNumber {
                            value: m.as_str().parse().unwrap(),
                            row: y,
                            range: m.range(),
                        }
                    })
                    .collect_vec()
            })
            .flatten()
            .collect_vec()
    }

    fn get_gears(&self) -> Vec<Point> {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let y = y.clone();
                row.iter().enumerate().filter_map(move |(x, c)| match c {
                    '*' => Some(Point { x, y }),
                    _ => None,
                })
            })
            .flatten()
            .collect_vec()
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<char> {
        self.get_adjacent_points(x, y)
            .iter()
            .map(|(x, y)| self.get(*x, *y))
            .collect_vec()
    }

    fn get_adjacent_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x_max = self.0.first().unwrap().len();
        let y_max = self.0.len();
        ADJACENT
            .iter()
            // relative -> absolute
            .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
            // only valid positions
            .filter(|(dx, dy)| *dx >= 0 && *dx < x_max as i32 && *dy >= 0 && *dy < y_max as i32)
            // fix type
            .map(|(dx, dy)| (dx as usize, dy as usize))
            .collect_vec()
    }
}

#[derive(Debug, Clone)]
struct PartNumber {
    value: u32,
    row: usize,
    range: Range<usize>,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn is_empty(c: char) -> bool {
    c == '.'
}
