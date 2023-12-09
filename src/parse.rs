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
