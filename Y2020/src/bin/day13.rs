extern crate derive_more;

extern crate itertools;

extern crate nom;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

struct Schedule(Option<i32>);
type BusId = i32;

// Part 1 //

fn solve1(schedules: &[Schedule], arrive_time: i32) -> i32 {
    let (bus_id, depart_time) = (arrive_time..)
        .find_map(|depart_time| {
            find_bus(schedules, depart_time).map(|bus_id| (bus_id, depart_time))
        })
        .expect("Result expected");
    bus_id * (depart_time - arrive_time)
}

fn find_bus(schedules: &[Schedule], depart_time: i32) -> Option<BusId> {
    schedules
        .iter()
        .find(|Schedule(schedule)| {
            if let Some(period) = schedule {
                depart_time % period == 0
            } else {
                false
            }
        })
        .map(|schedule| schedule.0.unwrap())
}

// Part 2 //

fn solve2(schedules: &[Schedule]) -> i64 {
    let mut schedules: Vec<ContestSchedule> = preprocess_contest(schedules);
    let result = calc_contest(&mut schedules);
    result.period - result.offset
}

// the_earliest_timestamp + offset == k * period
struct ContestSchedule {
    period: i64,
    offset: i64,
}

// Drop 'x' schedules, leaving only valid schedules
fn preprocess_contest(schedules: &[Schedule]) -> Vec<ContestSchedule> {
    schedules
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if let Schedule(Some(period)) = x {
                Some(ContestSchedule {
                    period: *period as i64,
                    offset: i as i64,
                })
            } else {
                None
            }
        })
        .collect()
}

fn calc_contest(schedules: &mut [ContestSchedule]) -> ContestSchedule {
    schedules.sort_by_key(|s| s.offset);
    schedules.reverse();
    // Clone first reference, since folding value shouldn't be a reference
    let first_schedule = ContestSchedule {
        period: schedules[0].period,
        offset: schedules[0].offset,
    };
    schedules
        .into_iter()
        .skip(1)
        .fold(first_schedule, |c1, c2| merge_schedule(&c1, c2))
}

// Merge two schedules into one equivalent schedule
fn merge_schedule(c1: &ContestSchedule, c2: &ContestSchedule) -> ContestSchedule {
    let (small, big) = if c1.period < c2.period {
        (c1, c2)
    } else {
        (c2, c1)
    };

    // t = a * small.period  - small.offset
    //   = b * big.period    - big.offset
    //   = c * merged_period - merged_offset
    let mut t = big.period - big.offset;
    loop {
        if (t + small.offset) % small.period == 0 {
            break;
        }
        t = t + big.period;
    }
    // merged_period = LCM(big.period, small.period)
    // Since both periods are prime numbers:
    let merged_period = big.period * small.period;
    let merged_offset = merged_period - t;
    ContestSchedule {
        period: merged_period,
        offset: merged_offset,
    }
}

// I/O //

fn main() {
    let input: Vec<String> = aoc::io::read_file_vec(Path::new("inputs/day13.txt"));
    let arrive_time: i32 = input[0].parse().expect("Number expected");
    let schedules: Vec<Schedule> = aoc::nom::unwrap_parsed(parse_schedules(&input[1]));
    println!("{}", solve1(&schedules, arrive_time));
    println!("{}", solve2(&schedules));
}

fn parse_schedules(input: &str) -> IResult<&str, Vec<Schedule>> {
    separated_list1(char(','), |i| -> IResult<&str, Schedule> {
        alt((
            map(char('x'), |_| Schedule(None)),
            map(aoc::nom::number, |n| Schedule(Some(n))),
        ))(i)
    })(input)
}
