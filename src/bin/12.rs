#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc::{get_input, report};
use itertools::Itertools;
use Condition::*;

// My original solution to part 1 took 2 minutes, part 2 would not have completed before heat death
// This solution required research, and was heavily influenced by ideas from other solutions

fn main() {
    let input = get_input("12");

    report(
        (|| part1(&input), Some(21), Some(7163)),
        (|| part2(&input), Some(525152), Some(17788038834112)),
    );
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (springs, summary) = l.split_once(" ").unwrap();
            let springs = springs.chars().map(Condition::from).collect_vec();
            let summary: Vec<usize> = summary.split(",").map(|s| s.parse().unwrap()).collect_vec();

            permutations(&mut HashMap::new(), &springs, None, &summary) as u64
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (springs, summary) = l.split_once(" ").unwrap();
            let springs = (0..5)
                .map(|_| springs)
                .join("?")
                .chars()
                .map(Condition::from)
                .collect_vec();

            let summary: Vec<usize> = summary.split(",").map(|s| s.parse().unwrap()).collect_vec();
            let summary = (0..5).flat_map(|_| &summary).copied().collect_vec();

            permutations(&mut HashMap::new(), &springs, None, &summary) as u64
        })
        .sum()
}

fn permutations(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    springs: &[Condition],
    group: Option<usize>,
    summary: &[usize],
) -> usize {
    if springs.is_empty() {
        return match (group, summary.len()) {
            // Reached with no more requirements
            (None, 0) => 1,
            // Reached the end matching the last group
            (Some(x), 1) if x == summary[0] => 1,
            // Reached the end unsuccessfully
            _ => 0,
        };
    }

    if group.is_some() && summary.is_empty() {
        return 0;
    }

    let key = (springs.len(), group.unwrap_or(0), summary.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let rest = &springs[1..];
    let matches = match (springs[0], group) {
        // group beginning
        (Broken, None) => permutations(cache, rest, Some(1), summary),
        // group continuing
        (Broken, Some(x)) => permutations(cache, rest, Some(x + 1), summary),
        // group ended too soon (must come before the next arm)
        (Working, Some(x)) if x != summary[0] => 0,
        // group ended correctly
        (Working, Some(_)) => permutations(cache, rest, None, &summary[1..]),
        // not in a group, continue
        (Working, None) => permutations(cache, rest, None, summary),
        // group ended by unknown
        (Unknown, Some(x)) if x == summary[0] => permutations(cache, rest, None, &summary[1..]),
        // group continuing by unknown
        (Unknown, Some(x)) => permutations(cache, rest, Some(x + 1), summary),
        // not in a group, bifurcate
        (Unknown, None) => {
            permutations(cache, rest, Some(1), summary) + permutations(cache, rest, None, summary)
        }
    };

    cache.insert(key, matches);
    matches
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    springs: Vec<Condition>,
    summary: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Working,
    Broken,
    Unknown,
}

impl From<char> for Condition {
    fn from(c: char) -> Self {
        match c {
            '.' => Condition::Working,
            '#' => Condition::Broken,
            '?' => Condition::Unknown,
            _ => panic!("invalid condition"),
        }
    }
}
