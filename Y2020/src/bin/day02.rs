extern crate nom;
use nom::character::complete::{alpha1, char, space0, space1};
use nom::{sequence, IResult};

use std::path::Path;

use advent_of_code_rust_2020 as aoc;

type Password = String;
#[derive(Debug)]
struct Policy {
    letter: char,
    num1: i32,
    num2: i32,
}

fn solve1(entries: &[(Password, Policy)]) -> i32 {
    entries.iter().filter(is_valid_1).count() as i32
}

fn is_valid_1(entry: &&(Password, Policy)) -> bool {
    let (password, policy) = entry;
    let count = password.chars().filter(|&c| c == policy.letter).count() as i32;
    policy.num1 <= count && count <= policy.num2
}

fn solve2(entries: &[(Password, Policy)]) -> i32 {
    entries.iter().filter(is_valid_2).count() as i32
}

fn is_valid_2(entry: &&(Password, Policy)) -> bool {
    let (password, policy) = entry;
    let first = password.chars().nth((policy.num1 - 1) as usize);
    let second = password.chars().nth((policy.num2 - 1) as usize);
    let c = policy.letter;
    match (first, second) {
        (Some(x), Some(y)) if (x == c) ^ (y == c) => true,
        _ => false,
    }
}

fn main() {
    let lines = aoc::io::read_file_line(Path::new("inputs/day02.txt"));
    let entries: Vec<(Password, Policy)> = lines
        .map(|s| aoc::nom::unwrap_parsed(parse_entry(&s)))
        .collect();
    println!("{}", solve1(&entries));
    println!("{}", solve2(&entries));
}

fn parse_policy<'a>(input: &'a str) -> IResult<&'a str, Policy> {
    let err =
        |i: &'a str| nom::Err::Failure(nom::error::Error::new(i, nom::error::ErrorKind::Char));
    sequence::tuple((
        aoc::nom::number,
        char('-'),
        aoc::nom::number,
        space1,
        alpha1,
    ))(input)
    .and_then(|(input, (num1, _, num2, _, alphas))| {
        if alphas.len() != 1 {
            Err(err(input))
        } else {
            let letter: char = alphas.chars().next().ok_or(err(input))?;
            Ok((
                input,
                Policy {
                    letter: letter,
                    num1: num1,
                    num2: num2,
                },
            ))
        }
    })
}

fn parse_entry(input: &str) -> IResult<&str, (Password, Policy)> {
    sequence::tuple((parse_policy, char(':'), space0, alpha1))(input)
        .and_then(|(input, (policy, _, _, password))| Ok((input, (String::from(password), policy))))
}
