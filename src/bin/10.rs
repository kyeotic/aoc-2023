#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::mem::swap;

use anyhow::{Context, Result};
use aoc::grid::{Grid, GridPoint};
use aoc::point::{CardinalDirection as Dir, Point};
use aoc::{get_input, report};
use itertools::Itertools;

fn main() {
    let input = get_input("10");

    report(
        (|| part1(&input), Some(4), Some(6886)),
        (|| part2(&input), None, None),
    );
}

fn part1(input: &str) -> u64 {
    let grid: Grid<Pipe> = Grid::from(input);
    let start = grid.iter().find(|p| p.value == Pipe::Start).unwrap();

    let mut travel = 1;
    let (mut next, to) = grid
        .get_neighbor_steps(&start)
        .into_iter()
        .find(|(g, step)| g.value.can_accept(&step.inverse()))
        .unwrap();

    let mut from = to.inverse();

    while next.value != Pipe::Start {
        let to = next.value.to(&from);
        next = grid.step(&next, to).unwrap();
        from = to.inverse();
        travel += 1;
    }

    // println!("final {travel:?}");

    travel / 2
}

fn part2(_input: &str) -> u64 {
    2
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Pipe {
    fn to(&self, from: &Dir) -> Dir {
        match self {
            Pipe::Vertical => Pipe::travel(from, Dir::Up, Dir::Down),
            Pipe::Horizontal => Pipe::travel(from, Dir::Left, Dir::Right),
            Pipe::NorthToEast => Pipe::travel(from, Dir::Up, Dir::Right),
            Pipe::NorthToWest => Pipe::travel(from, Dir::Up, Dir::Left),
            Pipe::SouthToWest => Pipe::travel(from, Dir::Down, Dir::Left),
            Pipe::SouthToEast => Pipe::travel(from, Dir::Down, Dir::Right),
            Pipe::Ground => panic!("cannot travel from ground"),
            Pipe::Start => panic!("cannot travel through start"),
        }
    }

    fn travel(from: &Dir, a: Dir, b: Dir) -> Dir {
        if from == &a {
            b
        } else {
            a
        }
    }

    fn can_accept(&self, from: &Dir) -> bool {
        match from {
            Dir::Up => {
                *self == Pipe::Vertical || *self == Pipe::NorthToEast || *self == Pipe::NorthToWest
            }
            Dir::Down => {
                *self == Pipe::Vertical || *self == Pipe::SouthToEast || *self == Pipe::SouthToWest
            }
            Dir::Left => {
                *self == Pipe::Horizontal
                    || *self == Pipe::NorthToWest
                    || *self == Pipe::SouthToWest
            }
            Dir::Right => {
                *self == Pipe::Horizontal
                    || *self == Pipe::NorthToEast
                    || *self == Pipe::SouthToEast
            }
        }
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthToEast,
            'J' => Pipe::NorthToWest,
            '7' => Pipe::SouthToWest,
            'F' => Pipe::SouthToEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("invalid pipe"),
        }
    }
}

// enum Pipe {
//     Vertical(Dir::Up, Dir::Down),
//     Horizontal,
//     NorthToEast,
//     NorthToWest,
//     SouthToWest,
//     SouthToEast,
//     Ground,
//     Start,
// }

// the starting points can `accept(from) because Pipe<From,To>
// so the pipe maps a boolean as well as a SOURCE-DESTINATION MAPPING`
