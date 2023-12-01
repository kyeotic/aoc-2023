#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::Itertools;
#[macro_use]
extern crate lazy_static;

fn main() {
    let input = get_input("01");

    let a: u32 = input.lines().map(extract_val).sum();
    // let a = 1;
    let b: u32 = input.lines().map(extract_named_val).sum();

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    assert_eq!(a, 54390);
    assert_eq!(b, 54277);
}

fn extract_val(line: &str) -> u32 {
    let digits = line.chars().filter(|c| c.is_numeric()).collect_vec();
    let a = digits.first().unwrap();
    let b = digits.last().unwrap();
    // println!("extracting {line} {a} {b}");
    format!("{a}{b}").parse::<u32>().unwrap()
}

lazy_static! {
    static ref NUMBERS: Vec<String> =
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",]
            .map(String::from)
            .to_vec();
}

fn extract_named_val(line: &str) -> u32 {
    extract_val(&replace_named(line))
}

fn replace_named(val: &str) -> String {
    let mut val = val.to_owned();
    for (i, num) in NUMBERS.iter().enumerate() {
        // println!("replacing {val} {num} {i}");
        val = val.replace(num, &format!("{}{}{}", num, (i + 1).to_string(), num));
    }
    val
}
