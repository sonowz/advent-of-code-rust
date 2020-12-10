extern crate derive_more;
use derive_more::{Add, FromStr, Sub};

use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

#[derive(Debug, FromStr, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
struct Jolt(i32);

// Part 1 //

fn solve1(jolts: &[Jolt]) -> i32 {
    let (diff1s, _, diff3s) = bucket_by_diff(jolts);
    (diff1s.len() * diff3s.len()) as i32
}

fn bucket_by_diff(jolts: &[Jolt]) -> (Vec<Jolt>, Vec<Jolt>, Vec<Jolt>) {
    let mut jolts = jolts.to_vec();
    jolts.sort();
    let device_jolt = *jolts.iter().max().unwrap() + Jolt(3);
    jolts.push(device_jolt);

    let (mut d1, mut d2, mut d3) = (Vec::new(), Vec::new(), Vec::new());
    jolts.iter().fold(Jolt(0), |prev, &curr| {
        match curr - prev {
            Jolt(1) => d1.push(curr),
            Jolt(2) => d2.push(curr),
            Jolt(3) => d3.push(curr),
            _ => panic!("No link between {:?} and {:?}!", prev, curr),
        };
        curr
    });
    (d1, d2, d3)
}

// Part 2 //

fn solve2(jolts: &[Jolt]) -> u64 {
    count_arrangements(jolts)
}

fn count_arrangements(jolts: &[Jolt]) -> u64 {
    let mut jolts = jolts.to_vec();
    jolts.sort();
    let device_jolt = *jolts.iter().max().unwrap() + Jolt(3);
    jolts.push(device_jolt);

    // Dynamic programming
    // State holds 3 previous entries as (jolt, count)
    // Compute 'count' from 'jolt'
    //   count[i] = if jolt[i] - jolt[i-1] <= 3 then count[i-1] else 0
    //            + if jolt[i] - jolt[i-2] <= 3 then count[i-2] else 0
    //            + if jolt[i] - jolt[i-3] <= 3 then count[i-3] else 0
    #[derive(Clone, Copy)]
    struct State {
        jolt: Jolt,
        count: u64,
    }
    impl State {
        fn new(jolt: Jolt, count: u64) -> Self {
            State { jolt, count }
        }
    }
    let init_state = (
        State::new(Jolt(-10), 0),
        State::new(Jolt(-10), 0),
        State::new(Jolt(0), 1),
    );
    let fold_result = jolts
        .iter()
        .fold(init_state, |(prev3, prev2, prev1), &curr_jolt| {
            let mut sum = 0;
            let mut add_connecting = |State { jolt, count }| {
                if curr_jolt - jolt <= Jolt(3) {
                    sum = sum + count
                }
            };
            add_connecting(prev1);
            add_connecting(prev2);
            add_connecting(prev3);
            let curr = State::new(curr_jolt, sum);
            (prev2, prev1, curr)
        });
    fold_result.2.count
}

// I/O //

fn main() {
    let jolts: Vec<Jolt> = aoc::io::read_file_vec(Path::new("inputs/day10.txt"));
    println!("{}", solve1(&jolts));
    println!("{}", solve2(&jolts));
}
