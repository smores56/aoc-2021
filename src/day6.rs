use std::collections::HashMap;

use anyhow::Context;

use crate::aoc::Aoc;

fn parse_fish(input: &str) -> anyhow::Result<HashMap<usize, usize>> {
    let ages = input
        .trim()
        .split(",")
        .map(|num| num.parse().context("Invalid fish age"));
    let mut fish = HashMap::new();

    for age in ages {
        *fish.entry(age?).or_default() += 1;
    }

    Ok(fish)
}

fn pass_a_day(fish: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut aged_fish = HashMap::new();
    for (age, quantity) in fish {
        if age == 0 {
            *aged_fish.entry(6).or_default() += quantity;
            *aged_fish.entry(8).or_default() += quantity;
        } else {
            *aged_fish.entry(age - 1).or_default() += quantity;
        }
    }

    aged_fish
}

pub struct Day6;

impl Aoc for Day6 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let mut fish = parse_fish(input)?;

        for _ in 0..80 {
            fish = pass_a_day(fish);
        }

        Ok(fish.values().sum())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let mut fish = parse_fish(input)?;

        for _ in 0..256 {
            fish = pass_a_day(fish);
        }

        Ok(fish.values().sum())
    }
}
