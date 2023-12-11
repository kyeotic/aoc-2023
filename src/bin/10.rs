#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap, HashSet};
use std::mem::swap;

use anyhow::{Context, Result};
use aoc::grid::{Grid, GridPoint};
use aoc::point::{CardinalDirection as Dir, Point};
use aoc::{get_input, report};
use indexmap::IndexSet;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let input = get_input("10");

    report(
        (|| part1(&input), Some(4), Some(6886)),
        (|| part2(&input), Some(4), Some(371)),
    );
}

fn part1(input: &str) -> u64 {
    (get_path(&Grid::from(input)).len() / 2) as u64
}

fn part2(input: &str) -> u64 {
    let grid: Grid<Pipe> = Grid::from(input);
    let path = get_path(&grid);
    let start = grid.iter().find(|p| p.value == Pipe::Start).unwrap();

    let conns = get_start_connections(&start, &grid);
    let correct_start = Pipe::iter()
        .find(|p| conns.iter().all(|(_, d)| p.can_accept(d)))
        .unwrap();

    grid.points
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter(|(i, p)| {
                    if path.contains(*p) {
                        return false;
                    };
                    let crosses = &row[0..*i]
                        .iter()
                        .filter(|rc| {
                            path.contains(*rc)
                                // The start needs to be replaced with the correct pipe for border detection
                                && is_border(if rc.value == Pipe::Start {
                                    &correct_start
                                } else {
                                    &rc.value
                                })
                        })
                        .count();

                    crosses % 2 == 1
                })
                .count() as u64
        })
        .sum::<u64>()
}

fn get_path(grid: &Grid<Pipe>) -> IndexSet<GridPoint<Pipe>> {
    let mut path: IndexSet<GridPoint<Pipe>> = IndexSet::new();
    let start = grid.iter().find(|p| p.value == Pipe::Start).unwrap();
    path.insert(start.clone());

    let conns = get_start_connections(&start, &grid);
    let (mut next, to) = conns.first().unwrap();

    let mut from = to.inverse();

    while next.value != Pipe::Start {
        path.insert(next.clone());
        let to = next.value.to(&from);
        next = grid.step(&next, to).unwrap();
        from = to.inverse();
    }

    path
}

fn get_start_connections(
    start: &GridPoint<Pipe>,
    grid: &Grid<Pipe>,
) -> Vec<(GridPoint<Pipe>, Dir)> {
    grid.get_neighbor_steps(&start)
        .into_iter()
        .filter(|(g, step)| g.value.can_accept(&step.inverse()))
        .collect_vec()
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    fn pairs(&self) -> (Dir, Dir) {
        match self {
            Pipe::Vertical => (Dir::Up, Dir::Down),
            Pipe::Horizontal => (Dir::Left, Dir::Right),
            Pipe::NorthToEast => (Dir::Up, Dir::Right),
            Pipe::NorthToWest => (Dir::Up, Dir::Left),
            Pipe::SouthToWest => (Dir::Down, Dir::Left),
            Pipe::SouthToEast => (Dir::Down, Dir::Right),
            Pipe::Ground => panic!("cannot travel from ground"),
            Pipe::Start => panic!("cannot travel through start"),
        }
    }

    fn to(&self, from: &Dir) -> Dir {
        let (a, b) = self.pairs();
        if from == &a {
            b
        } else {
            a
        }
    }

    fn can_accept(&self, from: &Dir) -> bool {
        let (a, b) = self.pairs();
        from == &a || from == &b
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

lazy_static! {
    static ref BORDERS: Vec<Pipe> = vec![Pipe::Vertical, Pipe::NorthToEast, Pipe::NorthToWest,];
}

fn is_border(p: &Pipe) -> bool {
    BORDERS.contains(p)
}
