extern crate derive_more;

use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, none_of, one_of};
use nom::combinator::{all_consuming, opt};
use nom::multi::{many1, many_m_n};
use nom::{sequence, IResult};

use std::fs::File;
use std::io::Read;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

#[derive(Debug)]
struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

// Part 1 //

fn solve1(passport_strings: &Vec<&str>) -> i32 {
    let passports: Vec<Passport> = passport_strings
        .iter()
        .filter_map(|ps| parse_passport(ps).ok().map(|(_, ps)| ps))
        .collect();
    passports.len() as i32
}

fn parse_passport<'a>(input: &'a str) -> IResult<&'a str, Passport> {
    all_consuming(permutation((
        parse_data("byr"),
        parse_data("iyr"),
        parse_data("eyr"),
        parse_data("hgt"),
        parse_data("hcl"),
        parse_data("ecl"),
        parse_data("pid"),
        opt(parse_data("cid")),
    )))(input)
    .and_then(|(input, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid))| {
        Ok((
            input,
            Passport {
                birth_year: byr,
                issue_year: iyr,
                expiration_year: eyr,
                height: hgt,
                hair_color: hcl,
                eye_color: ecl,
                passport_id: pid,
                country_id: cid,
            },
        ))
    })
}

// Parses /key:[^\s]+\s*/
fn parse_data(key: &'static str) -> impl Fn(&str) -> IResult<&str, String> {
    let to_str = |v: Vec<char>| -> String { v.iter().collect() };
    move |input| {
        sequence::tuple((tag(key), char(':'), many1(none_of(" \n")), multispace0))(input)
            .and_then(|(input, (_, _, value, _))| Ok((input, to_str(value))))
    }
}

// Part 2 //

fn solve2(passport_strings: &Vec<&str>) -> i32 {
    let passports: Vec<Passport> = passport_strings
        .iter()
        .filter_map(|ps| parse_passport(ps).ok().map(|(_, ps)| ps))
        .filter(|ps| passport_is_valid(&ps))
        .collect();
    passports.len() as i32
}

fn passport_is_valid(ps: &Passport) -> bool {
    let guarding = || {
        // Exploit '?' syntax of Option<()>
        // rather than 'if b { false }';
        let range =
            |s: &str, min, max| s.parse().ok().filter(|&x| min <= x && x <= max).map(|_| ());
        range(&ps.birth_year, 1920, 2002)?;
        range(&ps.issue_year, 2010, 2020)?;
        range(&ps.expiration_year, 2020, 2030)?;
        parse_height(&ps.height)
            .ok()
            .filter(|(_, height)| match height {
                Height::Cm(h) => 150 <= *h && *h <= 193,
                Height::In(h) => 59 <= *h && *h <= 76,
            })?;
        parse_hair_color(&ps.hair_color).ok()?;
        parse_eye_color(&ps.eye_color).ok()?;
        parse_passport_id(&ps.passport_id).ok()?;
        Some(())
    };
    guarding().is_some()
}

enum Height {
    Cm(i32),
    In(i32),
}

fn parse_height(input: &str) -> IResult<&str, Height> {
    aoc::nom::number(input).and_then(|(input, num)| match input {
        "cm" => Ok(("", Height::Cm(num))),
        "in" => Ok(("", Height::In(num))),
        _ => Err(aoc::nom::error(input, nom::error::ErrorKind::Char)),
    })
}

fn parse_hair_color(input: &str) -> IResult<&str, String> {
    let to_str = |v: Vec<char>| -> String { v.iter().collect() };
    sequence::tuple((char('#'), many_m_n(6, 6, one_of("0123456789abcdef"))))(input)
        .and_then(|(input, (_, v))| Ok((input, to_str(v))))
}

fn parse_eye_color(input: &str) -> IResult<&str, &str> {
    alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input)
}

fn parse_passport_id(input: &str) -> IResult<&str, Vec<char>> {
    many_m_n(9, 9, one_of("0123456789"))(input)
}

// I/O //

fn main() {
    let mut input = String::new();
    File::open(Path::new("inputs/day04.txt"))
        .expect("File open error")
        .read_to_string(&mut input)
        .expect("File read error");
    let passport_strings: Vec<&str> = input.split("\n\n").collect();
    println!("{}", solve1(&passport_strings));
    println!("{}", solve2(&passport_strings));
}
