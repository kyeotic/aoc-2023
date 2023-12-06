#![allow(dead_code, unused_imports, unreachable_code)]
#[macro_use]
extern crate lazy_static;
use std::{
    collections::HashMap,
    ops::{Range, RangeFrom},
    str::Split,
};

use aoc::{get_input, measure, report};
use itertools::Itertools;

fn main() {
    let input = get_input("05");

    report(
        (|| part1(input.clone()), Some(35), Some(26273516)),
        (|| part2(input.clone()), Some(45), Some(34039469)),
    );
}

fn part1(input: String) -> u32 {
    let (seeds, maps) = split_input(&input);
    let seeds = parse_seeds1(seeds.unwrap());
    let maps = parse_maps(maps);

    let seeded = seeds
        .iter()
        .map(|seed| {
            let mut cursor = *seed;

            // Walk through the maps
            for map in maps.clone() {
                let t = map.maps.iter().find(|m| m.source.contains(&cursor));
                if let Some(t) = t {
                    cursor = t.dest.start + (cursor - t.source.start);
                }
            }
            cursor
        })
        .collect_vec();

    seeded.iter().min().unwrap().clone()
}

fn part2(input: String) -> u32 {
    let (seeds, maps) = split_input(&input);
    let seeds = parse_seeds2(seeds.unwrap());
    let maps = parse_maps(maps);
    let maps = maps.iter().rev().collect_vec();

    // println!("s2 {:?}", seeds);

    // return 1;

    // This is WAY too slow and needs to be fixed
    let seeded = seeds
        .iter()
        .map(|r| r.clone().collect::<Vec<u32>>())
        .flatten()
        .map(|seed| {
            let mut cursor = seed;

            for map in maps.clone() {
                let t = map.maps.iter().find(|m| m.source.contains(&cursor));
                if let Some(t) = t {
                    cursor = t.dest.start + (cursor - t.source.start);
                }
                // println!("mapped {} to {}", map.name, cursor);
            }
            cursor
        })
        .collect_vec();

    // println!(
    //     "maps: {:?}",
    //     maps.iter().map(|m| m.name.clone()).collect_vec()
    // );

    // println!("seeded {:?}", seeded);

    seeded.iter().min().unwrap().clone()
}

fn split_input<'a>(input: &'a str) -> (Option<&'a str>, Split<'a, &str>) {
    let mut sections = input.split("\n\n");
    let seeds = sections.next();
    let maps = sections;

    (seeds, maps)
}

fn parse_seeds1(s: &str) -> Vec<u32> {
    s.replace("seeds: ", "")
        .split(" ")
        .map(|p| p.parse::<u32>().unwrap())
        .collect_vec()
}

fn parse_seeds2(s: &str) -> Vec<Range<u32>> {
    parse_seeds1(s)
        .chunks(2)
        .map(|r| (r[0]..(r[0] + r[1])))
        .collect_vec()
}

fn parse_maps<'a>(groups: impl Iterator<Item = &'a str>) -> Vec<AlmanacMap> {
    groups
        .map(|g| {
            let lines = g.lines().collect_vec();
            let name = lines[0];
            let maps = lines[1..]
                .iter()
                .map(|m| {
                    let (dest, source, range) = m
                        .split(" ")
                        .map(|p| p.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    MapRange::new(source, dest, range)
                })
                .collect_vec();
            AlmanacMap::new(name.to_string(), maps)
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct AlmanacMap {
    name: String,
    maps: Vec<MapRange>,
}

impl AlmanacMap {
    fn new(name: String, maps: Vec<MapRange>) -> Self {
        Self { name, maps }
    }
}

#[derive(Debug, Clone)]
struct MapRange {
    source: Range<u32>,
    dest: Range<u32>,
    range: u32,
}

impl MapRange {
    fn new(source: u32, dest: u32, range: u32) -> Self {
        Self {
            source: (source..(source + range)),
            dest: (dest..(dest + range)),
            range,
        }
    }
}
