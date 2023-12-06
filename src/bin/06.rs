#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use aoc::{get_input, parse::parse_spaced_nums, report};
use itertools::Itertools;
use std::iter::zip;

fn main() {
    let input = get_input("06");

    report(
        (|| part1(&input), Some(288), Some(393120)),
        (|| part2(&input), Some(71503), Some(36872656)),
    );
}

fn part1(input: &str) -> u32 {
    let (times, distances) = input
        .split("\n")
        .map(|l| parse_spaced_nums::<u32>(&l.replace("Time:", "").replace("Distance:", "")))
        .collect_tuple()
        .unwrap();

    let wins = zip(times, distances)
        .map(|(t, d)| count_race_wins(t as u64, d as u64))
        .collect_vec();

    wins.iter().product()
}

fn part2(input: &str) -> u32 {
    let (time, distance) = input
        .split("\n")
        .map(|l| {
            l.replace("Time:", "")
                .replace("Distance:", "")
                .replace(" ", "")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    count_race_wins(time, distance)
}

fn count_race_wins(time: u64, distance: u64) -> u32 {
    (1..=time)
        .map(|t| (time - t) * t)
        .filter(|d| d > &distance)
        .count() as u32
}
