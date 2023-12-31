#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate lazy_static;
use aoc::{get_input, report};
use counter::Counter;
use itertools::Itertools;
use std::cmp::Ordering;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn main() {
    let input = get_input("07");
    report(
        (|| part1(&input), Some(6440), Some(253313241)),
        (|| part2(&input), Some(5905), Some(253362743)),
    );
}

fn part1(input: &str) -> u32 {
    calc_winners(input, None)
}

fn part2(input: &str) -> u32 {
    calc_winners(input, Some('J'))
}

fn calc_winners(input: &str, wild: Option<char>) -> u32 {
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split(" ").collect_tuple().unwrap();
            let hand = hand.chars().collect_vec();
            let bid: u32 = bid.parse().unwrap();
            (hand, bid)
        })
        .sorted_by(|a, b| camel_rank(a, b, wild))
        .rev()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CamelHand {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn camel_rank(a: &(Vec<char>, u32), b: &(Vec<char>, u32), wild: Option<char>) -> Ordering {
    let a_hand = camel_hand(&a.0, wild);
    let b_hand = camel_hand(&b.0, wild);

    match a_hand.cmp(&b_hand) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            for (i, c) in a.0.iter().enumerate() {
                match card_rank(c, wild).cmp(&card_rank(&b.0[i], wild)) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => continue,
                }
            }
            panic!("equal hands are invalid")
        }
    }
}

fn camel_hand(hand: &Vec<char>, wild: Option<char>) -> CamelHand {
    let common = Counter::<&char, u32>::init(hand).most_common();
    let max_unique = common
        .iter()
        .filter(|k| wild.is_none() || k.0 != &wild.unwrap())
        .map(|k| k.1)
        .max()
        .unwrap_or(0);
    // let j = 0;
    let j = hand
        .iter()
        .filter(|c| wild.is_some() && c == &&wild.unwrap())
        .count() as u32;

    if max_unique + j == 5 || j == 5 {
        CamelHand::FiveKind
    } else if max_unique + j == 4 || j == 4 {
        CamelHand::FourKind
    } else if is_full_house(&common, j) {
        CamelHand::FullHouse
    } else if max_unique + j == 3 || j == 3 {
        CamelHand::ThreeKind
    } else if common.iter().filter(|k| k.1 == 2).count() == 2 {
        // There is no way to have TwoPair with a joker
        CamelHand::TwoPair
    } else if common.iter().filter(|k| k.1 == 2).count() == 1 || j == 1 {
        CamelHand::OnePair
    } else {
        // There is no way to have HighCard with a joker
        CamelHand::HighCard
    }
}

fn is_full_house(common: &Vec<(&char, u32)>, wild_count: u32) -> bool {
    match wild_count {
        0 => common.iter().any(|k| k.1 == 3) && common.iter().any(|k| k.1 == 2),
        1 => common.iter().filter(|k| k.1 == 2).count() == 2,
        // 2 Jokers can't make FullHouse because any other pair would be FourKind
        // 3 jokers is always FourKind
        _ => false,
    }
}

fn card_rank(c: &char, wild: Option<char>) -> u32 {
    if wild.is_some() && c == &wild.unwrap() {
        CARDS.len() as u32
    } else {
        CARDS.iter().position(|i| c == i).unwrap() as u32
    }
}
