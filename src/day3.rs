use crate::aoc::Aoc;
use anyhow::Context;

fn parse_binary(number: &str) -> anyhow::Result<usize> {
    usize::from_str_radix(number, 2).context("Invalid binary")
}

pub struct Day3;

impl Aoc for Day3 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let numbers = input.lines().collect::<Vec<_>>();

        let bit_count = numbers[0].len();
        let mut one_counts = vec![0; bit_count];

        for binary in &numbers {
            for (index, bit) in binary.chars().enumerate() {
                if bit == '1' {
                    one_counts[index] += 1;
                }
            }
        }

        let gamma_rate: String = one_counts
            .iter()
            .map(|count| if count * 2 > numbers.len() { '1' } else { '0' })
            .collect();
        let epsilon_rate: String = one_counts
            .iter()
            .map(|count| if count * 2 <= numbers.len() { '1' } else { '0' })
            .collect();

        Ok(parse_binary(&gamma_rate)? * parse_binary(&epsilon_rate)?)
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let numbers = input.lines().collect::<Vec<_>>();

        let mut oxygen_rating_candidates = numbers.clone();
        let mut current_bit = 0;
        while oxygen_rating_candidates.len() > 1 {
            let one_count = oxygen_rating_candidates
                .iter()
                .filter(|candidate| candidate.chars().nth(current_bit) == Some('1'))
                .count();
            let most_common_bit = if one_count * 2 >= oxygen_rating_candidates.len() {
                '1'
            } else {
                '0'
            };
            oxygen_rating_candidates.drain_filter(|candidate| {
                candidate.chars().nth(current_bit) == Some(most_common_bit)
            });

            if oxygen_rating_candidates.len() > 1 {
                current_bit += 1;
            }
        }
        let oxygen_rating = oxygen_rating_candidates[0];

        let mut carbon_dioxide_rating_candidates = numbers.clone();
        current_bit = 0;
        while carbon_dioxide_rating_candidates.len() > 1 {
            let one_count = carbon_dioxide_rating_candidates
                .iter()
                .filter(|candidate| candidate.chars().nth(current_bit) == Some('1'))
                .count();
            let least_common_bit = if one_count * 2 < carbon_dioxide_rating_candidates.len() {
                '1'
            } else {
                '0'
            };
            carbon_dioxide_rating_candidates.drain_filter(|candidate| {
                candidate.chars().nth(current_bit) == Some(least_common_bit)
            });

            if carbon_dioxide_rating_candidates.len() > 1 {
                current_bit += 1;
            }
        }
        let carbon_dioxide_rating = carbon_dioxide_rating_candidates[0];

        Ok(parse_binary(oxygen_rating)? * parse_binary(carbon_dioxide_rating)?)
    }
}
