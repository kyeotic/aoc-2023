#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use aoc::{get_input, iter::intersection, report};
use itertools::Itertools;

fn main() {
    let input = get_input("04");
    // let lines = input.lines();
    let cards: Vec<Card> = input
        .lines()
        .map(|l| l.parse::<Card>().unwrap())
        .collect_vec();

    let a: u32 = cards.iter().map(|c| c.score).sum();
    let b = part2(cards);

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    assert_eq!(a, 20829);
    assert_eq!(b, 12648035);
}

fn part2(cards: Vec<Card>) -> u32 {
    let mut duplicates: HashMap<u32, u32> = HashMap::new();

    for c in cards.clone() {
        duplicates.insert(c.id, 1);
    }

    for c in cards.clone() {
        let m = c.wins;
        if m == 0 {
            continue;
        }

        let n = duplicates.get(&c.id).unwrap().clone();
        for i in (c.id + 1)..=(c.id + m) {
            duplicates.entry(i).and_modify(|x| *x += n);
        }
    }

    duplicates.values().sum()
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    wins: u32,
    score: u32,
}

impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, body) = s.split(":").collect_tuple().unwrap();
        let (winners, numbers) = body.split("|").map(parse_nums).collect_tuple().unwrap();

        let wins = intersection(&winners, &numbers).len() as u32;

        Ok(Self {
            id: header.replace("Card ", "").trim().parse().unwrap(),
            wins,
            score: 2_u32.pow((wins - 1) as u32),
        })
    }
}

fn parse_nums(s: &str) -> Vec<u32> {
    s.split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse().unwrap())
        .collect_vec()
}
