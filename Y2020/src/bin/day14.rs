extern crate derive_more;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

const N: usize = 36;

#[derive(Debug)]
enum ProgramLine {
    Mask(BitMask),
    Mem((u64, u64)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mask {
    Z,
    O,
    X,
}
impl From<char> for Mask {
    fn from(c: char) -> Self {
        match c {
            '0' => Mask::Z,
            '1' => Mask::O,
            'X' => Mask::X,
            _ => panic!("Invalid bitmask"),
        }
    }
}
type BitMask = [Mask; N];

// Part 1 //

fn solve1(program: &[ProgramLine]) -> u64 {
    let mut mem = HashMap::new();
    let init_state = State {
        mem: &mut mem,
        mask: [Mask::X; N],
    };
    let final_state = program.iter().fold(init_state, run_programline);
    final_state.mem.values().sum()
}

fn run_programline<'a>(s: State<'a>, line: &ProgramLine) -> State<'a> {
    match line {
        ProgramLine::Mask(new_mask) => State {
            mask: new_mask.clone(),
            mem: s.mem,
        },
        ProgramLine::Mem((index, value)) => {
            s.mem.insert(*index, mask_value(&s.mask, *value));
            s
        }
    }
}

fn mask_value(mask: &BitMask, value: u64) -> u64 {
    let masked: BinCollect = BinIter::new(value)
        .enumerate()
        .map(|(i, b)| match mask[i] {
            Mask::Z => 0,
            Mask::O => 1,
            Mask::X => b,
        })
        .collect();
    masked.0
}

type Memory = HashMap<u64, u64>;
struct State<'a> {
    mem: &'a mut Memory,
    mask: BitMask,
}

// Iterate u64 as 36-bit binary number ([0, 1, 1, ...])
struct BinIter {
    val: u64,
    pow: i32,
}
impl BinIter {
    fn new(x: u64) -> Self {
        BinIter {
            val: x,
            pow: N as i32,
        }
    }
}
impl Iterator for BinIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.pow = self.pow - 1;
        if self.pow < 0 {
            None
        } else {
            let denom = 2u64.pow(self.pow as u32);
            Some((self.val / denom) % 2)
        }
    }
}

// Collect binary numbers into u64
struct BinCollect(u64);
impl FromIterator<u64> for BinCollect {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut val = 0;
        for x in iter {
            val = 2 * val + x;
        }
        BinCollect(val)
    }
}

// Part 2 //

fn solve2(program: &[ProgramLine]) -> u64 {
    let mut mem = HashMap::new();
    let init_state = State {
        mem: &mut mem,
        mask: [Mask::X; N],
    };
    let final_state = program.iter().fold(init_state, run_programline2);
    final_state.mem.values().sum()
}

fn run_programline2<'a>(s: State<'a>, line: &ProgramLine) -> State<'a> {
    match line {
        ProgramLine::Mask(new_mask) => State {
            mask: new_mask.clone(),
            mem: s.mem,
        },
        ProgramLine::Mem((index, value)) => {
            let addr_mask = mask_index(&s.mask, *index);
            MaskIter::new(addr_mask).for_each(|addr| {
                s.mem.insert(addr, *value);
            });
            s
        }
    }
}

fn mask_index(mask: &BitMask, index: u64) -> BitMask {
    let mut new_mask = [Mask::O; N];
    BinIter::new(index).enumerate().for_each(|(i, b)| {
        new_mask[i] = match mask[i] {
            Mask::Z => {
                if b == 0 {
                    Mask::Z
                } else {
                    Mask::O
                }
            }
            Mask::O => Mask::O,
            Mask::X => Mask::X,
        }
    });
    new_mask
}

// Iterate bitmask into u64 w.r.t X as wildcard
#[derive(Debug)]
struct MaskIter {
    // If bitmask == 11X0X,
    base: u64,       // 11000
    x_pow: Vec<u64>, // [4, 1]
    cnt: u64,        // 32
    iter: u64,
}
impl MaskIter {
    fn new(x: BitMask) -> Self {
        let get_pow = |i: usize| 2u64.pow(N as u32 - i as u32 - 1);
        let (x_pow, bases): (Vec<_>, Vec<_>) =
            x.iter().enumerate().partition(|&(_, &b)| b == Mask::X);
        let cnt = 2u64.pow(x_pow.len() as u32);
        let x_pow = x_pow.iter().map(|&(i, _)| get_pow(i)).collect();
        let base = bases
            .iter()
            .map(|&(i, b)| if let Mask::O = b { get_pow(i) } else { 0 })
            .sum();
        MaskIter {
            base,
            x_pow,
            cnt,
            iter: 0,
        }
    }
}
impl Iterator for MaskIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter == self.cnt {
            return None;
        }
        let get_pow = |i: usize| self.cnt / 2u64.pow(i as u32 + 1);
        let offset: u64 = self
            .x_pow
            .iter()
            .enumerate()
            .map(
                |(i, &pow)| {
                    if get_pow(i) & self.iter != 0 {
                        pow
                    } else {
                        0
                    }
                },
            )
            .sum();
        self.iter = self.iter + 1;
        Some(self.base + offset)
    }
}

// I/O //

fn main() {
    let input = aoc::io::read_file_line(Path::new("inputs/day14.txt"));
    let program: Vec<ProgramLine> = input.map(parse_programline).collect();
    println!("{}", solve1(&program));
    println!("{}", solve2(&program));
}

fn parse_programline(line: String) -> ProgramLine {
    if line.starts_with("mask") {
        ProgramLine::Mask(parse_mask(&line[7..]))
    } else if line.starts_with("mem") {
        let splitted: Vec<_> = line.split(|c| c == '[' || c == ']' || c == ' ').collect();
        let index = splitted[1].parse().unwrap();
        let value = splitted[4].parse().unwrap();
        ProgramLine::Mem((index, value))
    } else {
        panic!("Should be 'mask' or 'mem'")
    }
}

fn parse_mask(mask_str: &str) -> BitMask {
    let mut mask = [Mask::O; N];
    mask_str.chars().enumerate().for_each(|(i, c)| {
        mask[i] = c.into();
    });
    mask
}
