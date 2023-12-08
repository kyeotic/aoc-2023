#![allow(dead_code, unused_imports, unreachable_code)]
#[macro_use]
extern crate lazy_static;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::str::FromStr;

use aoc::{get_input, report};
use itertools::Itertools;

fn main() {
    let input = get_input("08");

    report(
        (|| part1(&input), Some(2), Some(20093)),
        (|| part2(&input), Some(6), Some(22103062509257)),
    );
}

fn part1(_input: &str) -> u64 {
    let (steps, nodes) = parse_input(_input);

    let mut current = nodes.get("AAA").unwrap();
    let mut path = vec![current];

    while current.name != "ZZZ" {
        let step = steps.get((path.len() - 1) % steps.len()).unwrap();
        let next = current.walk(step);
        current = nodes.get(next).unwrap();
        path.push(&current);
    }

    (path.len() as u64) - 1
}

fn part2(input: &str) -> u64 {
    let (steps, nodes) = parse_input(input);

    let mut cycles: Vec<u32> = Vec::new();

    let ghosts = nodes
        .values()
        .filter_map(|n| if n.name.ends_with("A") { Some(n) } else { None })
        .collect_vec();

    for ghost in ghosts {
        let mut n = 0;
        let mut node = ghost;

        while !node.name.ends_with("Z") {
            let step = steps.get(n % steps.len()).unwrap();
            let next = node.walk(step);
            node = nodes.get(next).unwrap();
            n += 1;
        }

        cycles.push(n as u32);
    }

    let n = cycles
        .iter()
        .map(|n| *n as u64)
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();

    n
}

fn parse_input(input: &str) -> (Vec<Step>, IndexMap<String, Node>) {
    let (steps, nodes) = input.split("\n\n").collect_tuple().unwrap();
    let steps: Vec<Step> = steps.chars().map(Step::from).collect();
    let nodes: IndexMap<_, _> = nodes
        .lines()
        .map(|l| l.parse::<Node>().unwrap())
        .map(|n| (n.name.to_owned(), n))
        .into_iter()
        .collect();

    (steps, nodes)
}

#[derive(Debug, Copy, Clone)]
enum Step {
    Left,
    Right,
}

impl From<char> for Step {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid step"),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn walk(&self, step: &Step) -> &str {
        match step {
            Step::Left => &self.left,
            Step::Right => &self.right,
        }
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, children) = s.split("=").collect_tuple().context("game")?;
        let name = name.trim().to_owned();
        let (left, right) = children
            .replace(&['(', ')'], "")
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect_tuple()
            .unwrap();

        Ok(Self { name, left, right })
    }
}
