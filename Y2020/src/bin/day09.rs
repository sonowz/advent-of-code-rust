extern crate derive_more;

use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

type Number = i64;

// Part 1 //

fn solve1(data: &[Number]) -> i64 {
    let failed_index = (25..data.len())
        .find(|&j| {
            let i = j - 25;
            !check_sum(&data[i..j], &data[j])
        })
        .expect("Should be an answer");
    data[failed_index]
}

fn check_sum(preambles: &[Number], target: &Number) -> bool {
    preambles
        .iter()
        .enumerate()
        .find(|&(i, x)| {
            preambles
                .iter()
                .enumerate()
                .find(|&(j, y)| i != j && x + y == *target)
                .is_some()
        })
        .is_some()
}

// Part 2 //

fn solve2(data: &[Number], answer_1: Number) -> i64 {
    let answer_slice = (2..)
        .find_map(|window_size| find_windowed_sum(data, answer_1, window_size))
        .expect("Should be an answer");
    answer_slice.iter().min().unwrap() + answer_slice.iter().max().unwrap()
}

// Returns contiguous set
fn find_windowed_sum(data: &[Number], target: Number, window_size: usize) -> Option<&[Number]> {
    let mut sum: Number = data[0..window_size].into_iter().sum();
    (window_size..data.len()).find_map(|j| {
        // Sliding window
        sum = sum - data[j - window_size] + data[j];
        if sum == target {
            Some(&data[j - window_size + 1..j + 1])
        } else {
            None
        }
    })
}

// I/O //

fn main() {
    let data: Vec<Number> = aoc::io::read_file_vec(Path::new("inputs/day09.txt"));
    let answer_1 = solve1(&data);
    println!("{}", answer_1);
    println!("{}", solve2(&data, answer_1));
}
