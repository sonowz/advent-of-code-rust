extern crate derive_more;

use std::path::Path;
use std::str::FromStr;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Instr {
    op: Op,
    arg: i32,
}
impl FromStr for Instr {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_instr(s).ok_or(std::fmt::Error)
    }
}

type Program = Vec<Instr>;

struct ProgramState {
    pc: usize,
    acc: i32,
}

// Part 1 //

fn solve1(program: &Program) -> i32 {
    run_until_loop(program).acc
}

fn step(program: &Program, st: ProgramState) -> ProgramState {
    let instr = &program[st.pc];
    ProgramState {
        pc: match instr.op {
            Op::Jmp => (st.pc as i32 + instr.arg) as usize,
            _ => st.pc + 1,
        },
        acc: match instr.op {
            Op::Acc => st.acc + instr.arg,
            _ => st.acc,
        },
    }
}

fn run_until_loop(program: &Program) -> ProgramState {
    let mut visited = vec![false; program.len()];
    let mut st = ProgramState { pc: 0, acc: 0 };
    loop {
        visited[st.pc] = true;
        st = step(program, st);
        if visited[st.pc] {
            break;
        }
    }
    st
}

// Part 2 //

fn solve2(program: Program) -> i32 {
    let mut fixed = FixedProgram::new(program);
    loop {
        fixed.fix_next();
        let state = run_until_loop_or_terminate(&fixed.prog);
        if check_terminated(&fixed.prog, &state) {
            return state.acc;
        }
    }
}

struct FixedProgram {
    prog: Program,
    fix: Option<usize>,
}
impl FixedProgram {
    fn new(prog: Program) -> Self {
        FixedProgram {
            prog: prog,
            fix: None,
        }
    }
    fn fix_next(&mut self) {
        let mut nfix = 0;
        // Undo previous fix
        if let Some(i) = self.fix {
            self.flip_op(i);
            nfix = i;
        }
        // Find next fix target
        nfix = self
            .prog
            .iter()
            .enumerate()
            .skip(nfix + 1)
            .find(|(_, &Instr { op, arg: _ })| op == Op::Jmp || op == Op::Nop)
            .expect("Should have one!")
            .0;
        // Do fix
        self.flip_op(nfix);
        self.fix = Some(nfix);
    }
    fn flip_op(&mut self, i: usize) {
        self.prog[i].op = match self.prog[i].op {
            Op::Jmp => Op::Nop,
            Op::Nop => Op::Jmp,
            Op::Acc => Op::Acc,
        }
    }
}

fn run_until_loop_or_terminate(program: &Program) -> ProgramState {
    let mut visited = vec![false; program.len()];
    let mut st = ProgramState { pc: 0, acc: 0 };
    loop {
        visited[st.pc] = true;
        st = step(program, st);
        if check_terminated(program, &st) || visited[st.pc] {
            break;
        }
    }
    st
}

fn check_terminated(program: &Program, st: &ProgramState) -> bool {
    st.pc >= program.len()
}

// I/O //

fn main() {
    let program: Program = aoc::io::read_file_vec(Path::new("inputs/day08.txt"));
    println!("{}", solve1(&program));
    println!("{}", solve2(program));
}

fn parse_instr(s: &str) -> Option<Instr> {
    let op = match s.get(0..3)? {
        "acc" => Op::Acc,
        "jmp" => Op::Jmp,
        "nop" => Op::Nop,
        _ => return None,
    };
    let arg = s.get(4..)?.parse().ok()?;
    Some(Instr { op: op, arg: arg })
}
