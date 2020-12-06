use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::Iterator;
use std::path::Path;
use std::str::FromStr;

use crate::grid::Grid;

pub fn read_file_line(path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("File open error");
    let reader = BufReader::new(file);
    reader.lines().map(|r| r.expect("File read error"))
}

pub fn read_file_vec<T>(path: &Path) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    read_file_line(path)
        .map(|s| s.parse().expect("Parse error"))
        .collect()
}

pub fn read_file_grid<T>(path: &Path) -> Grid<T>
where
    T: From<char>,
{
    let vec: Vec<Vec<_>> = read_file_line(path)
        .map(|s| s.chars().map(T::from).collect())
        .collect();
    Grid::from(vec)
}

// Returns strings (with newline), separated by blank line
pub fn read_file_blankline(path: &Path) -> impl Iterator<Item = String> {
    let mut input = String::new();
    File::open(path)
        .expect("File open error")
        .read_to_string(&mut input)
        .expect("File read error");
    let lines_vec: Vec<_> = input.split("\n\n").map(|s| String::from(s)).collect();
    lines_vec.into_iter()
}
