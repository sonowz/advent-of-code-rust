extern crate derive_more;

use std::fmt;
use std::fmt::Display;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;
use aoc::grid::Grid;

// Type Declarations //

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            _ => panic!("Invalid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => ".",
                Tile::Empty => "L",
                Tile::Occupied => "#",
            }
        )
        .unwrap();
        Ok(())
    }
}

// Part 1 //

fn solve1(grid: &Grid<Tile>) -> i32 {
    let mut grid: Grid<Tile> = Grid::from(grid.vec.clone());
    loop {
        let new_grid = step(&grid);
        if grid.vec == new_grid.vec {
            break;
        }
        grid = new_grid
    }
    grid.vec
        .iter()
        .map(|l| l.iter().filter(|&x| *x == Tile::Occupied).count() as i32)
        .sum()
}

fn step(grid: &Grid<Tile>) -> Grid<Tile> {
    let (row_len, col_len) = grid.size();
    let next_grid: Vec<Vec<Tile>> = (0..row_len)
        .map(|i| (0..col_len).map(|j| next_tile(grid, i, j)).collect())
        .collect();
    Grid::from(next_grid)
}

fn next_tile(grid: &Grid<Tile>, i: usize, j: usize) -> Tile {
    match grid.vec[i][j] {
        Tile::Empty => {
            if adjacent_occupied(grid, i, j) == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
        Tile::Occupied => {
            if adjacent_occupied(grid, i, j) >= 4 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
        Tile::Floor => Tile::Floor,
    }
}

fn adjacent_occupied(grid: &Grid<Tile>, i: usize, j: usize) -> usize {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    deltas
        .iter()
        .map(|&(dx, dy)| {
            if let Some(tile) = get_tile(grid, i, j, dx, dy) {
                if tile == Tile::Occupied {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

fn get_tile(grid: &Grid<Tile>, i: usize, j: usize, dx: i32, dy: i32) -> Option<Tile> {
    let x = (i as i32) + dx;
    let y = (j as i32) + dy;
    if (x < 0) || (y < 0) {
        return None;
    };
    Some(*grid.vec.get(x as usize)?.get(y as usize)?)
}

// Part 2 //

fn solve2(grid: &Grid<Tile>) -> i32 {
    let mut grid: Grid<Tile> = Grid::from(grid.vec.clone());
    loop {
        let new_grid = step2(&grid);
        if grid.vec == new_grid.vec {
            break;
        }
        grid = new_grid
    }
    grid.vec
        .iter()
        .map(|l| l.iter().filter(|&x| *x == Tile::Occupied).count() as i32)
        .sum()
}

fn step2(grid: &Grid<Tile>) -> Grid<Tile> {
    let (row_len, col_len) = grid.size();
    let next_grid: Vec<Vec<Tile>> = (0..row_len)
        .map(|i| (0..col_len).map(|j| next_tile2(grid, i, j)).collect())
        .collect();
    Grid::from(next_grid)
}

fn next_tile2(grid: &Grid<Tile>, i: usize, j: usize) -> Tile {
    match grid.vec[i][j] {
        Tile::Empty => {
            if adjacent_occupied2(grid, i, j) == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
        Tile::Occupied => {
            if adjacent_occupied2(grid, i, j) >= 5 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
        Tile::Floor => Tile::Floor,
    }
}

fn adjacent_occupied2(grid: &Grid<Tile>, i: usize, j: usize) -> usize {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    deltas
        .iter()
        .map(|&(dx, dy)| {
            let (mut i, mut j) = (i, j);
            loop {
                if let Some(tile) = get_tile(grid, i, j, dx, dy) {
                    match tile {
                        Tile::Occupied => break 1,
                        Tile::Empty => break 0,
                        Tile::Floor => {
                            i = ((i as i32) + dx) as usize;
                            j = ((j as i32) + dy) as usize;
                        }
                    }
                } else {
                    break 0;
                }
            }
        })
        .sum()
}

// I/O //

fn main() {
    let grid: Grid<Tile> = aoc::io::read_file_grid(Path::new("inputs/day11.txt"));
    println!("{}", solve1(&grid));
    println!("{}", solve2(&grid));
}
