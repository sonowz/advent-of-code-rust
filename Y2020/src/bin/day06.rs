extern crate derive_more;

extern crate itertools;
use itertools::Itertools;

use std::collections::HashSet;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

type Answers = Vec<char>;
type GroupAnswers = Vec<Answers>;

// Part 1 //

fn solve1(group_answers: &Vec<GroupAnswers>) -> i32 {
    group_answers
        .iter()
        .map(collect_group_answers)
        .map(|ans| ans.len() as i32)
        .sum()
}

fn collect_group_answers(group_answer: &GroupAnswers) -> Answers {
    let mut answer_set = HashSet::new();
    group_answer.iter().for_each(|answers| {
        answers.iter().for_each(|&c| {
            answer_set.insert(c);
        })
    });
    answer_set.into_iter().collect()
}

// Part 2 //

fn solve2(group_answers: &Vec<GroupAnswers>) -> i32 {
    group_answers
        .iter()
        .map(collect_group_answers_2)
        .map(|ans| ans.len() as i32)
        .sum()
}

fn collect_group_answers_2(group_answer: &GroupAnswers) -> Answers {
    let to_set = |vec: &Vec<char>| -> HashSet<char> { vec.into_iter().cloned().collect() };

    let group_answer_sets: HashSet<char> = group_answer
        .iter()
        .map(to_set)
        .fold1(|answer_set, x_answer_set| {
            // Compute intersection of answer sets
            answer_set.intersection(&x_answer_set).cloned().collect()
        })
        .expect("'group_answer' must not be empty");
    group_answer_sets.into_iter().collect()
}

// I/O //

fn main() {
    let group_answers: Vec<GroupAnswers> =
        aoc::io::read_file_blankline(Path::new("inputs/day06.txt"))
            .map(parse_group_answers)
            .collect();
    println!("{}", solve1(&group_answers));
    println!("{}", solve2(&group_answers));
}

fn parse_group_answers(s: String) -> GroupAnswers {
    // GroupAnswers == Vec<Vec<char>>
    s.split("\n").map(|line| line.chars().collect()).collect()
}
