use std::collections::HashMap;

use crate::aoc::Aoc;

fn validate_chunk(chunk: &str) -> Result<Vec<char>, char> {
    let chunk_ends: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();
    let mut chunk_scope = vec![];

    for bracket in chunk.chars() {
        if let Some(closer) = chunk_ends.get(&bracket) {
            chunk_scope.push(*closer);
        } else {
            match chunk_scope.pop() {
                None => return Ok(vec![]),
                Some(closer) if closer != bracket => {
                    return Err(bracket);
                }
                _ => {}
            }
        }
    }

    Ok(chunk_scope.into_iter().rev().collect())
}

pub struct Day10;

impl Aoc for Day10 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let bracket_scores: HashMap<char, usize> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
            .into_iter()
            .collect();

        Ok(input
            .trim()
            .lines()
            .map(|chunk| {
                validate_chunk(chunk)
                    .err()
                    .and_then(|closer| bracket_scores.get(&closer).cloned())
                    .unwrap_or_default()
            })
            .sum())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let bracket_scores: HashMap<char, usize> = [(')', 1), (']', 2), ('}', 3), ('>', 4)]
            .into_iter()
            .collect();

        let mut scores: Vec<usize> = input
            .trim()
            .lines()
            .flat_map(|chunk| validate_chunk(chunk).ok())
            .map(|missing_brackets| {
                missing_brackets
                    .into_iter()
                    .fold(0, |total_score, bracket| {
                        let bracket_score =
                            bracket_scores.get(&bracket).cloned().unwrap_or_default();
                        total_score * 5 + bracket_score
                    })
            })
            .collect();
        scores.sort();

        Ok(scores[(scores.len() - 1) / 2])
    }
}
