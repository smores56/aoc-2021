// use std::cmp::{max, min};
use std::ops::RangeInclusive;

use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

use crate::aoc::Aoc;

pub struct Day17;

fn parse_int(input: &str) -> IResult<&str, isize> {
    let (input, sign) = take_while_m_n(0, 1, |c| c == '-')(input)?;
    let (input, value) = map_res(digit1, |digit_str: &str| digit_str.parse::<isize>())(input)?;

    Ok((input, value * (if sign == "-" { -1 } else { 1 })))
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<isize>> {
    let (input, start) = parse_int(input)?;
    let (input, _range) = tag("..")(input)?;
    let (input, end) = parse_int(input)?;

    Ok((input, start..=end))
}

fn parse_target_area(input: &str) -> IResult<&str, (RangeInclusive<isize>, RangeInclusive<isize>)> {
    let (input, _intro) = tag("target area: x=")(input)?;
    let (input, x_range) = parse_range(input)?;
    let (input, _comma) = tag(", y=")(input)?;
    let (input, y_range) = parse_range(input)?;

    Ok((input, (x_range, y_range)))
}

fn x_position(x_vel: isize, steps: isize) -> isize {
    if steps >= x_vel.abs() {
        x_vel * (x_vel.abs() + 1) / 2
    } else {
        x_vel.signum() * (2 * x_vel.abs() - steps + 1) * steps / 2
    }
}

fn y_position(y_vel: isize, steps: isize) -> isize {
    (2 * y_vel - steps + 1) * steps / 2
}

impl Aoc for Day17 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        // let (_rest, (_x_range, y_range)) = parse_target_area(input)?;

        // (0..=1000).flat_map(|steps| (0..=1000).map(|y_vel| (steps, y_vel)).map(|(steps, y_vel|))

        // (0..1_000_000).map(|steps| {

        // })
        // let (_rest, (x_range, y_range)) =
        // parse_target_area(input).context("Failed to parse input")?;

        Ok(0)
    }

    fn part2(&self, _input: &str) -> anyhow::Result<usize> {
        unimplemented!()
    }
}
