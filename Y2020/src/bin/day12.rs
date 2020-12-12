#![feature(bindings_after_at)]
extern crate derive_more;

use std::path::Path;
use std::str::FromStr;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

struct Instr {
    action: Action,
    value: i32,
}
impl FromStr for Instr {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = std::fmt::Error;
        let action = Action::from(s.chars().nth(0).ok_or(err)?);
        let value = s[1..].parse().map_err(|_| err)?;
        Ok(Instr { action, value })
    }
}

enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}
impl From<char> for Action {
    fn from(c: char) -> Self {
        match c {
            'N' => Action::North,
            'S' => Action::South,
            'E' => Action::East,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            'F' => Action::Forward,
            _ => panic!("Invalid tile"),
        }
    }
}

struct State {
    pos: (i32, i32),
    dir: Dir,
}
impl State {
    fn new(pos: (i32, i32), dir: Dir) -> Self {
        State { pos, dir }
    }
}
#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

// Part 1 //

fn solve1(instrs: &[Instr]) -> i32 {
    let init_state = State::new((0, 0), Dir::East);
    let last_state = instrs
        .iter()
        .fold(init_state, |st, instr| move_ship(st, instr));
    i32::abs(last_state.pos.0) + i32::abs(last_state.pos.1)
}

fn move_ship(st: State, instr: &Instr) -> State {
    let add = |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2);
    match instr.action {
        Action::North => State::new(add(st.pos, (instr.value, 0)), st.dir),
        Action::South => State::new(add(st.pos, (-instr.value, 0)), st.dir),
        Action::East => State::new(add(st.pos, (0, instr.value)), st.dir),
        Action::West => State::new(add(st.pos, (0, -instr.value)), st.dir),
        Action::Left => State::new(st.pos, rotate_left(st.dir, instr.value)),
        Action::Right => State::new(st.pos, rotate_left(st.dir, -instr.value)),
        Action::Forward => {
            let st_dir = st.dir;
            move_ship(
                st,
                &Instr {
                    action: dir_to_action(st_dir),
                    value: instr.value,
                },
            )
        }
    }
}

fn rotate_left(dir: Dir, angle: i32) -> Dir {
    if angle < 0 {
        rotate_left(dir, angle + 360)
    } else if angle == 0 {
        dir
    } else if angle < 90 {
        panic!("Should be multiple of 90 degrees!")
    } else {
        let new_dir = match dir {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        };
        rotate_left(new_dir, angle - 90)
    }
}

fn dir_to_action(dir: Dir) -> Action {
    match dir {
        Dir::North => Action::North,
        Dir::South => Action::South,
        Dir::East => Action::East,
        Dir::West => Action::West,
    }
}

// Part 2 //

fn solve2(instrs: &[Instr]) -> i32 {
    let init_state = State2::new((0, 0), (1, 10));
    let last_state = instrs.iter().fold(init_state, |st, instr| {
        println!("{:?}", (st.pos, st.wp));
        move_waypoint(st, instr)
    });
    i32::abs(last_state.pos.0) + i32::abs(last_state.pos.1)
}

struct State2 {
    pos: (i32, i32),
    wp: (i32, i32),
}
impl State2 {
    fn new(pos: (i32, i32), wp: (i32, i32)) -> Self {
        State2 { pos, wp }
    }
}

fn move_waypoint(st: State2, instr: &Instr) -> State2 {
    let add = |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2);
    let mul = |k, (x, y)| (k * x, k * y);
    match instr.action {
        Action::North => State2::new(st.pos, add(st.wp, (instr.value, 0))),
        Action::South => State2::new(st.pos, add(st.wp, (-instr.value, 0))),
        Action::East => State2::new(st.pos, add(st.wp, (0, instr.value))),
        Action::West => State2::new(st.pos, add(st.wp, (0, -instr.value))),
        Action::Left => State2::new(st.pos, rotate_wp_left(st.wp, instr.value)),
        Action::Right => State2::new(st.pos, rotate_wp_left(st.wp, -instr.value)),
        Action::Forward => State2::new(add(st.pos, mul(instr.value, st.wp)), st.wp),
    }
}

fn rotate_wp_left(wp @ (x, y): (i32, i32), angle: i32) -> (i32, i32) {
    if angle < 0 {
        rotate_wp_left(wp, angle + 360)
    } else if angle == 0 {
        wp
    } else if angle < 90 {
        panic!("Should be multiple of 90 degrees!")
    } else {
        rotate_wp_left((y, -x), angle - 90)
    }
}

// I/O //

fn main() {
    let instructions: Vec<Instr> = aoc::io::read_file_vec(Path::new("inputs/day12.txt"));
    println!("{}", solve1(&instructions));
    println!("{}", solve2(&instructions));
}
