use crate::aoc::Aoc;
use anyhow::Context;

pub struct Day1;

fn parse_depths(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .lines()
        .map(|line| line.parse().context("invalid depth"))
        .collect()
}

impl Aoc for Day1 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let depths = parse_depths(input)?;

        Ok(depths
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let depths = parse_depths(input)?;

        let sums_of_three_wide_windows = depths
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<usize>>();

        Ok(sums_of_three_wide_windows
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count())
    }
}
