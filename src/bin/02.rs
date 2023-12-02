#![allow(dead_code, unused_imports)]
use anyhow::{Context, Result};
use aoc::{get_input, report};
use itertools::Itertools;
use std::cmp::max;
use std::str::FromStr;

fn main() {
    let input = get_input("02");

    let a_max = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect_vec();
    let a: u32 = games
        .iter()
        .filter(|g| g.has_max(&a_max))
        .map(|g| g.id)
        .sum();
    let b: u32 = games.iter().map(|g| g.get_min().power()).sum();

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    assert_eq!(a, 2369);
    assert_eq!(b, 66363);
}

struct Game {
    id: u32,
    reveals: Vec<Cubes>,
}

impl Game {
    fn has_max(&self, max: &Cubes) -> bool {
        self.reveals
            .iter()
            .all(|r| r.blue <= max.blue && r.red <= max.red && r.green <= max.green)
    }

    fn get_min(&self) -> Cubes {
        self.reveals
            .iter()
            .copied()
            .reduce(|acc, e| Cubes {
                red: max(acc.red, e.red),
                blue: max(acc.blue, e.blue),
                green: max(acc.green, e.green),
            })
            .unwrap()
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, reveals) = s.split(":").collect_tuple().context("game")?;

        Ok(Self {
            id: header.replace("Game ", "").parse()?,
            reveals: reveals
                .split(";")
                .map(|r| r.parse().context("reveals").unwrap())
                .collect(),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl FromStr for Cubes {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s.split(",").map(|c| c.trim()).collect_vec();

        Ok(Self {
            red: Cubes::get_color("red", &colors),
            blue: Cubes::get_color("blue", &colors),
            green: Cubes::get_color("green", &colors),
        })
    }
}

impl Cubes {
    fn get_color(name: &str, search: &Vec<&str>) -> u32 {
        if let Some(c) = search.iter().find(|c| c.contains(name)) {
            c.replace(name, "").trim().parse().unwrap()
        } else {
            0
        }
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}
