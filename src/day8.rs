use std::collections::HashSet;

use crate::aoc::Aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Signal {
    segments: HashSet<char>,
}

fn parse_display(word: &str) -> Signal {
    Signal {
        segments: word.chars().collect(),
    }
}

fn parse_signals(input: &str) -> Vec<(Vec<Signal>, Vec<Signal>)> {
    input
        .lines()
        .map(|line| {
            let (input_list, output_list) = line.split_once(" | ").unwrap_or_default();
            let input_signals = input_list
                .split_ascii_whitespace()
                .map(parse_display)
                .collect();
            let output_signals = output_list
                .split_ascii_whitespace()
                .map(parse_display)
                .collect();

            (input_signals, output_signals)
        })
        .collect()
}

fn find_segment<F: Fn(&&Signal) -> bool>(
    signals: &[Signal],
    name: &str,
    predicate: F,
) -> anyhow::Result<Signal> {
    signals
        .iter()
        .find(predicate)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Missing segment {}", name))
}

fn determine_order_for_input(signals: &[Signal]) -> anyhow::Result<[Signal; 10]> {
    let one = find_segment(signals, "one", |signal| signal.segments.len() == 2)?;
    let four = find_segment(signals, "four", |signal| signal.segments.len() == 4)?;
    let seven = find_segment(signals, "seven", |signal| signal.segments.len() == 3)?;
    let eight = find_segment(signals, "eight", |signal| signal.segments.len() == 7)?;
    let nine = find_segment(signals, "nine", |signal| {
        signal.segments.len() == 6 && signal.segments.is_superset(&four.segments)
    })?;
    let three = find_segment(signals, "three", |signal| {
        signal.segments.len() == 5 && signal.segments.is_superset(&seven.segments)
    })?;
    let six = find_segment(signals, "six", |signal| {
        signal.segments.len() == 6 && signal.segments.union(&one.segments).count() == 7
    })?;
    let five = find_segment(signals, "five", |signal| {
        signal.segments.len() == 5 && signal.segments.is_subset(&six.segments)
    })?;
    let two = find_segment(signals, "two", |signal| {
        signal.segments.len() == 5 && signal.segments.union(&four.segments).count() == 7
    })?;
    let zero = find_segment(signals, "zero", |signal| {
        signal.segments.len() == 6 && signal != &&six && signal != &&nine
    })?;

    Ok([zero, one, two, three, four, five, six, seven, eight, nine])
}

pub struct Day8;

impl Aoc for Day8 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        Ok(input
            .lines()
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .skip_while(|word| word != &"|")
                    .skip(1)
            })
            .filter(|word| [2, 3, 4, 7].contains(&word.len()))
            .count())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let signal_groups = parse_signals(input);

        let outputs = signal_groups
            .iter()
            .map(|(inputs, outputs)| {
                let order = determine_order_for_input(inputs)?;
                let digits = outputs
                    .iter()
                    .map(|output| {
                        order
                            .iter()
                            .enumerate()
                            .find(|(_index, signal)| &signal.segments == &output.segments)
                            .map(|(index, _signal)| index)
                            .ok_or_else(|| anyhow::anyhow!("Missing digit: {:?}", order))
                    })
                    .collect::<anyhow::Result<Vec<usize>>>()?;
                let full_output = digits.iter().fold(0, |full, digit| full * 10 + digit);

                Ok(full_output)
            })
            .collect::<anyhow::Result<Vec<usize>>>()?;

        Ok(outputs.iter().sum())
    }
}
