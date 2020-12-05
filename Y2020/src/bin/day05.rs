extern crate derive_more;
use derive_more::Into;

use std::convert::Into;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

type SeatBin = Vec<Dir>;
#[derive(Debug)]
struct SeatPos(i32, i32);
#[derive(Into, PartialEq, Eq, PartialOrd, Ord)]
struct SeatId(i32);

enum Dir {
    Front,
    Back,
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'F' => Dir::Front,
            'B' => Dir::Back,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid tile"),
        }
    }
}

// Part 1 //

fn solve1(seat_bins: &Vec<SeatBin>) -> i32 {
    seat_bins
        .iter()
        .map(bin_to_pos)
        .map(pos_to_id)
        .max()
        .unwrap()
        .0
}

fn bin_to_pos(bin: &SeatBin) -> SeatPos {
    // 0-index
    let (mut row_min, mut row_max) = (0, 127);
    let (mut col_min, mut col_max) = (0, 7);
    bin.iter().for_each(|dir| match dir {
        Dir::Front => row_max = row_min + (row_max - row_min) / 2,
        Dir::Back => row_min = row_max - (row_max - row_min) / 2,
        Dir::Left => col_max = col_min + (col_max - col_min) / 2,
        Dir::Right => col_min = col_max - (col_max - col_min) / 2,
    });
    assert_eq!(row_min, row_max);
    assert_eq!(col_min, col_max);
    SeatPos(row_min, col_min)
}

fn pos_to_id(pos: SeatPos) -> SeatId {
    SeatId(8 * pos.0 + pos.1)
}

// Part 2 //

fn solve2(seat_bins: &Vec<SeatBin>) -> i32 {
    let ids: Vec<_> = seat_bins.iter().map(bin_to_pos).map(pos_to_id).collect();
    find_my_seat(ids).0
}

fn find_my_seat(ids: Vec<SeatId>) -> SeatId {
    let mut seats_reserved = [false; 8 * 128];

    for id in ids.into_iter() {
        seats_reserved[i32::from(id) as usize] = true;
    }
    let (my_seat_id, _) = seats_reserved
        .iter()
        .enumerate()
        .skip_while(|&(_, &r)| r == false) // Skip empty back seats
        .skip_while(|&(_, &r)| r == true) // Find non-reserved seat
        .next()
        .unwrap();
    SeatId(my_seat_id as i32)
}

// I/O //

fn main() {
    let seat_bins: Vec<SeatBin> = aoc::io::read_file_line(Path::new("inputs/day05.txt"))
        .map(parse_seat)
        .collect();
    println!("{}", solve1(&seat_bins));
    println!("{}", solve2(&seat_bins));
}

fn parse_seat(s: String) -> Vec<Dir> {
    assert_eq!(s.len(), 10);
    s.chars().map(Into::into).take(10).collect()
}
