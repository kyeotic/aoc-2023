use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

pub fn parse_spaced_nums<T>(str: &str) -> Vec<T>
where
    T: FromStr<Err = ParseIntError>,
{
    parse_nums(str, " ")
}

pub fn parse_nums<T>(str: &str, delimiter: &str) -> Vec<T>
where
    T: FromStr<Err = ParseIntError>,
{
    str.split(delimiter)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<T>().unwrap())
        .collect_vec()
}

pub fn as_paragraphs(s: &str) -> Vec<&str> {
    s.split("\n\n").collect_vec()
}

// Use a Grid<char> if you need coordinates or traversal
pub fn as_char_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

pub fn char_grid_to_string(grid: &Vec<Vec<char>>) -> String {
    grid.iter().map(|r| r.iter().join("")).join("\n")
}
