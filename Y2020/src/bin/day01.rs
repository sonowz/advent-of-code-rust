extern crate derive_more;
use derive_more::{Add, FromStr, Mul};

use std::path::Path;

use advent_of_code_rust_2020 as aoc;

#[derive(FromStr, Add, Mul, PartialEq, Eq, Clone, Copy)]
#[mul(forward)]
struct Entry(i32);

fn solve1(entries: &[Entry]) -> i32 {
    let pairs = || entries.iter().enumerate();
    let (i, j) = pairs()
        .find_map(|(i, &x)| {
            pairs().find_map(|(j, &y)| {
                if i != j && x + y == Entry(2020) {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .expect("Should have answer");
    (entries[i] * entries[j]).0
}

fn solve2(entries: &[Entry]) -> i32 {
    let pairs = || entries.iter().enumerate();
    let diff_3 = |i, j, k| i != j && j != k && k != i;
    let (i, j, k) = pairs()
        .find_map(|(i, &x)| {
            pairs().find_map(|(j, &y)| {
                pairs().find_map(|(k, &z)| {
                    if diff_3(i, j, k) && x + y + z == Entry(2020) {
                        Some((i, j, k))
                    } else {
                        None
                    }
                })
            })
        })
        .expect("Should have answer");
    (entries[i] * entries[j] * entries[k]).0
}

fn main() {
    let entries: Vec<Entry> = aoc::io::read_file_vec(Path::new("inputs/day01.txt"));
    println!("{}", solve1(&entries));
    println!("{}", solve2(&entries));
}
