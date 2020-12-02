extern crate nom;
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt, recognize};
use nom::{sequence, IResult};

use std::str::FromStr;

pub fn unwrap_parsed<T>(result: IResult<&str, T>) -> T {
    match result {
        Ok(("", x)) => x,
        Ok((s, _)) => panic!(format!("Non-exhaustive parsing: {}", s)),
        Err(e) => panic!(format!("{}", e)),
    }
}

pub fn number<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(
        recognize(sequence::tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(input)
}
