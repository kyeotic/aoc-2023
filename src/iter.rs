use std::collections::HashSet;
use std::hash::Hash;

pub fn intersection<T: Eq + Hash + Copy>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let unique_a = a.iter().copied().collect::<HashSet<_>>();
    let unique_b = b.iter().copied().collect::<HashSet<_>>();

    unique_a.intersection(&unique_b).copied().collect()
}
