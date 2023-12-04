#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::ops::Range;

use aoc::point::Point;
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
        .filter(|pn| {
            pn.get_points()
                .iter()
                .map(|p| manual.get_adjacent(p))
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
            let adj_parts = parts
                .iter()
                .filter(|pn| pn.get_points().iter().any(|p| p.is_adjacent_diagonal(g)))
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

    fn get(&self, p: &Point) -> char {
        self.0
            .get(p.y as usize)
            .unwrap()
            .get(p.x as usize)
            .unwrap()
            .to_owned()
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
                    '*' => Some(Point::new_u(x, y)),
                    _ => None,
                })
            })
            .flatten()
            .collect_vec()
    }

    fn get_adjacent(&self, p: &Point) -> Vec<char> {
        self.get_adjacent_points(p)
            .iter()
            .map(|p| self.get(p))
            .collect_vec()
    }

    fn get_adjacent_points(&self, p: &Point) -> Vec<Point> {
        p.get_adjacent_diagonal()
            .iter()
            .filter(|p| self.is_inside(p))
            .copied()
            .collect()
    }

    fn is_inside(&self, p: &Point) -> bool {
        let x_max = self.0.first().unwrap().len();
        let y_max = self.0.len();
        p.x >= 0 && p.x < x_max as i32 && p.y >= 0 && p.y < y_max as i32
    }
}

#[derive(Debug, Clone)]
struct PartNumber {
    value: u32,
    row: usize,
    range: Range<usize>,
}

impl PartNumber {
    fn get_points(&self) -> Vec<Point> {
        self.range
            .clone()
            .map(|x| Point::new_u(x, self.row))
            .collect()
    }
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn is_empty(c: char) -> bool {
    c == '.'
}
