extern crate derive_more;

use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

type Forest = aoc::grid::Grid<Tile>;
#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Tree,
            _ => panic!("Invalid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Empty => ".",
            Tile::Tree => "#",
        };
        write!(f, "{}", s).unwrap();
        Ok(())
    }
}

// Part 1 //

fn solve1(forest: &Forest) -> i32 {
    let dx = 3;
    let (count, _) = forest.vec.iter().fold((0, 0), |(cnt, x), line: &Vec<_>| {
        let cnt = cnt + if is_tree(line, x) { 1 } else { 0 };
        let x = x + dx;
        (cnt, x)
    });
    count
}

fn is_tree(line: &[Tile], n: i32) -> bool {
    *line.iter().cycle().nth(n as usize).unwrap() == Tile::Tree
}

// Part 2 //

fn solve2(forest: &Forest) -> i64 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let tree_counts = slopes.iter().map(|&(dx, dy)| {
        forest
            .vec
            .iter()
            .step_by(dy as usize)
            .fold((0, 0), |(cnt, x), line: &Vec<_>| {
                let cnt = cnt + if is_tree(line, x) { 1 } else { 0 };
                let x = x + dx;
                (cnt, x)
            })
            .0
    });
    tree_counts.product()
}

// I/O //

fn main() {
    let forest: Forest = aoc::io::read_file_grid(Path::new("inputs/day03.txt"));
    println!("{}", solve1(&forest));
    println!("{}", solve2(&forest));
}
