#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use anyhow::{Context, Result};
use aoc::{get_input, report};
use indexmap::IndexMap;
use itertools::Itertools;

fn main() {
    let input = get_input("15");

    report(
        (|| part1(&input), Some(1320), Some(511416)),
        (|| part2(&input), Some(145), Some(290779)),
    );
}

fn part1(input: &str) -> u64 {
    input.split(",").map(hash).sum()
}

fn part2(input: &str) -> u64 {
    let mut boxes: IndexMap<u64, IndexMap<String, u64>> = IndexMap::new();
    input.split(",").for_each(|step| {
        let (label, lens) = step.split_once(&['=', '-']).unwrap();

        let boxx = boxes.entry(hash(&label)).or_insert_with(|| IndexMap::new());

        match lens {
            "" => boxx.shift_remove(label),
            l => boxx.insert(label.to_owned(), l.parse().unwrap()),
        };
    });

    boxes
        .iter()
        .map(|(box_i, b)| {
            b.iter()
                .enumerate()
                .map(|(i, x)| (*box_i + 1) * (i as u64 + 1) * x.1)
        })
        .flatten()
        .sum()
}

fn hash(s: &str) -> u64 {
    s.as_bytes()
        .iter()
        .fold(0, |acc, n| ((acc + (*n as u64)) * 17) % 256)
}
