#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aoc::grid::{Grid, GridPoint};
use aoc::point::CardinalDirection as Dir;
use aoc::point::{CardinalDirection::*, Point};
use aoc::{get_input, report};
use itertools::Itertools;
use Cave::*;

fn main() {
    let input = get_input("16");

    report(
        (|| part1(&input), Some(46), Some(7979)),
        (|| part2(&input), Some(51), Some(8437)),
    );
}

fn part1(input: &str) -> u64 {
    let grid: Grid<Cave> = Grid::from(input);
    get_energized(&grid, grid.get(0, 0).unwrap(), Right)
}

fn part2(input: &str) -> u64 {
    let grid: Grid<Cave> = Grid::from(input);
    (0..grid.points.len())
        .into_iter()
        .map(|y| {
            [
                (grid.get_u(0, y).unwrap(), Left),
                (grid.get_u(grid.size_x() as usize, y).unwrap(), Right),
            ]
        })
        .flatten()
        .chain(
            (0..grid.size_x() as usize)
                .into_iter()
                .map(|x| {
                    [
                        (grid.get_u(x, 0).unwrap(), Down),
                        (grid.get_u(x, grid.size_y() as usize).unwrap(), Up),
                    ]
                })
                .flatten(),
        )
        .into_iter()
        .map(|(point, dir)| get_energized(&grid, point, dir))
        .max()
        .unwrap()
}

fn get_energized(grid: &Grid<Cave>, start: GridPoint<Cave>, dir: Dir) -> u64 {
    let mut energized: HashMap<Point, HashSet<Dir>> = HashMap::new();
    let mut beams = vec![Beam::new(start, dir)];

    while beams.len() > 0 {
        beams = beams
            .iter()
            .map(|beam| {
                let e = energized
                    .entry(beam.point.into())
                    .or_insert_with(|| HashSet::new());
                if e.contains(&beam.dir) {
                    return Vec::new();
                }
                e.insert(beam.dir.clone());

                let to = beam.point.value.to(&beam.dir.inverse());
                to.iter()
                    .filter_map(|t| {
                        grid.step(&beam.point, *t)
                            .and_then(|p| Some(Beam::new(p, t.clone())))
                    })
                    .collect_vec()
            })
            .flatten()
            .collect_vec();
    }

    energized.len() as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    point: GridPoint<Cave>,
    dir: Dir,
}

impl Beam {
    fn new(point: GridPoint<Cave>, dir: Dir) -> Self {
        Self { point, dir }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitterVertical,
    SplitterHorizontal,
}

impl Cave {
    fn travel(&self) -> ((Dir, Dir), (Dir, Dir)) {
        match self {
            MirrorLeft => ((Down, Right), (Up, Left)),
            MirrorRight => ((Left, Down), (Up, Right)),
            _ => ((Up, Down), (Left, Right)),
        }
    }

    fn splits(&self, from: &Dir) -> Option<(Dir, Dir)> {
        match self {
            SplitterVertical if *from == Left || *from == Right => Some((Up, Down)),
            SplitterHorizontal if *from == Up || *from == Down => Some((Left, Right)),
            _ => None,
        }
    }

    fn to(&self, from: &Dir) -> Vec<Dir> {
        if let Some(s) = self.splits(from) {
            return vec![s.0, s.1];
        }

        let path = self.travel();
        let pair = if path.0 .0 == *from || path.0 .1 == *from {
            path.0
        } else {
            path.1
        };

        let (a, b) = pair;
        vec![if from == &a { b } else { a }]
    }
}

impl From<char> for Cave {
    fn from(c: char) -> Self {
        match c {
            '.' => Empty,
            '/' => MirrorLeft,
            '\\' => MirrorRight,
            '|' => SplitterVertical,
            '-' => SplitterHorizontal,
            _ => panic!("invalid Cave"),
        }
    }
}
