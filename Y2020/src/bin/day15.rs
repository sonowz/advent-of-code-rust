extern crate derive_more;

use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

const N: usize = 30000000;

#[derive(Debug)]
struct History {
    hist: Vec<i32>,
    last_number: i32,
    epoch: i32,
}

// Part 1 //

fn solve1(starting_numbers: &[i32]) -> i32 {
    let mut history = init_history(starting_numbers);
    let offset = history.epoch;
    // Use 'nth' method from 'Iterator' trait
    history.nth(2020 - offset as usize).unwrap()
}

fn init_history(starting_numbers: &[i32]) -> History {
    assert_ne!(starting_numbers.len(), 0);
    let mut history = vec![0; N];
    starting_numbers
        .iter()
        .enumerate()
        .take(starting_numbers.len() - 1) // Exclude last number
        .for_each(|(i, &x)| history[x as usize] = (i + 1) as i32);
    History {
        hist: history,
        last_number: *starting_numbers.last().unwrap(),
        epoch: starting_numbers.len() as i32,
    }
}

// Calculate next number
fn step_history(history: &mut History) -> i32 {
    let last_number = history.last_number;
    let last_number_spoken = history.hist[last_number as usize];
    let new_number = if last_number_spoken == 0 {
        0
    } else {
        history.epoch - last_number_spoken
    };
    history.hist[last_number as usize] = history.epoch;
    history.last_number = new_number;
    history.epoch = history.epoch + 1;
    last_number
}

impl Iterator for History {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(step_history(self))
    }
}

// Part 2 //

fn solve2(starting_numbers: &[i32]) -> i32 {
    let mut history = init_history(starting_numbers);
    let offset = history.epoch;
    history.nth(30000000 - offset as usize).unwrap()
}

// I/O //

fn main() {
    let input: String = aoc::io::read_file_line(Path::new("inputs/day15.txt"))
        .next()
        .unwrap();
    let starting_numbers: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    println!("{}", solve1(&starting_numbers));
    println!("{}", solve2(&starting_numbers));
}
