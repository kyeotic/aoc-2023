#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use anyhow::{Context, Result};
use aoc::{get_input, parse::parse_spaced_nums, report};
use itertools::Itertools;

fn main() {
    let input = get_input("09");
    let histories = input.lines().map(parse_spaced_nums::<i64>).collect_vec();

    report(
        (|| part1(&histories), Some(114), Some(1853145119)),
        (|| part2(&histories), Some(2), Some(923)),
    );
}

fn part1(histories: &[Vec<i64>]) -> i64 {
    histories
        .iter()
        .map(|h| {
            let mut diffs = vec![h.clone()];

            while !diffs.last().unwrap().iter().all(|n| n == &0) {
                diffs.push(get_diffs(diffs.last().unwrap()))
            }

            let mut next = 0;
            for diff in diffs.iter_mut().rev().skip(1) {
                next = diff.last().unwrap() + next;
                diff.push(next);
            }

            diffs.first().unwrap().last().unwrap().clone()
        })
        .sum()
}

fn part2(histories: &[Vec<i64>]) -> i64 {
    part1(
        &histories
            .iter()
            .map(|v| v.iter().rev().copied().collect_vec())
            .collect_vec(),
    )
}

fn get_diffs(vec: &[i64]) -> Vec<i64> {
    vec.iter().tuple_windows().map(|(a, b)| b - a).collect_vec()
}
