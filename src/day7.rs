use anyhow::Context;

use crate::aoc::Aoc;

pub struct Day7;

fn parse_positions(input: &str) -> anyhow::Result<Vec<isize>> {
    input
        .trim()
        .split(",")
        .map(|pos| pos.parse().context("Invalid crab position"))
        .collect()
}

impl Aoc for Day7 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let positions = parse_positions(input)?;

        let min_pos = *positions.iter().min().context("No minimum position")? as usize;
        let max_pos = *positions.iter().max().context("No maximum position")? as usize;

        (min_pos..=max_pos)
            .map(|pos| {
                positions
                    .iter()
                    .map(|p| (p - (pos as isize)).abs() as usize)
                    .sum()
            })
            .min()
            .context("No optimal distance")
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let positions = parse_positions(input)?;

        println!("positions: {:?}", positions.len());

        let min_pos = *positions.iter().min().context("No minimum position")? as u128;
        let max_pos = *positions.iter().max().context("No maximum position")? as u128;

        Ok((min_pos..=max_pos)
            .filter_map(|pos| {
                positions.iter().try_fold::<u128, _, _>(0, |sum, p| {
                    let distance = p.checked_sub(pos as isize)?.checked_abs()? as u128;
                    let cost = distance.checked_mul(distance.checked_sub(1)?)?;
                    sum.checked_add(cost)
                })
            })
            .min()
            .context("No optimal distance")? as usize)
    }
}
