#[macro_use]
extern crate lazy_static;
use std::fmt::Display;
use std::time::Instant;
use std::{env, fs};
use time_humanize::HumanTime;

pub mod grid;
pub mod iter;
pub mod point;

pub fn get_input(day: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("inputs").join(format!("{}.in", day));

    fs::read_to_string(filepath).unwrap()
}

pub fn report<T, F, K>(a: (F, Option<T>, Option<T>), b: (K, Option<T>, Option<T>))
where
    T: Display + std::cmp::PartialEq,
    F: FnOnce() -> T,
    K: FnOnce() -> T,
{
    report_part("Part 1", a);
    report_part("Part 2", b);
}

fn report_part<T, F>(label: &str, part: (F, Option<T>, Option<T>))
where
    T: Display + std::cmp::PartialEq,
    F: FnOnce() -> T,
{
    let (result, time) = measure(part.0);

    let matches = if part.2.is_some() && result == part.2.unwrap() {
        "✅"
    } else if part.1.is_some() && &result == part.1.as_ref().unwrap() {
        "🟦"
    } else if part.1.is_none() {
        "🟨"
    } else {
        "❌"
    };

    println!(
        "{label: <6}: {result: <20} (in {: <25}) {: <10}",
        time, matches
    );
}

pub fn measure<F, T>(func: F) -> (T, String)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let r = func();
    let reported = HumanTime::from(start.elapsed());
    (
        r,
        reported.to_text_en(
            time_humanize::Accuracy::Precise,
            time_humanize::Tense::Present,
        ),
    )
}
