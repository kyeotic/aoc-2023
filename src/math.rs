use std::cmp;

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn is_between<T: Ord + std::fmt::Display>(val: T, a: T, b: T) -> bool {
    let low = cmp::min(&a, &b);
    let high = cmp::max(&a, &b);

    // println!("between {val} {low} {high}: {}", low < &val && &val < high);

    low < &val && &val < high
}
