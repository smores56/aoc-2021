use std::collections::HashMap;

use anyhow::Context;

use crate::aoc::Aoc;

struct Rule {
    start: char,
    end: char,
    insert: char,
}

impl Rule {
    fn parse(rule: &str) -> Self {
        let chars = rule.chars().collect::<Vec<char>>();

        Self {
            start: chars[0],
            end: chars[1],
            insert: chars[6],
        }
    }
}

struct Polymer {
    links: HashMap<char, HashMap<char, usize>>,
}

impl Polymer {
    fn new(template: &str) -> Self {
        let components: Vec<char> = template.chars().collect();
        let links = components.array_windows::<2>().fold(
            HashMap::<char, HashMap<char, usize>>::new(),
            |mut links, &[start, end]| {
                *links.entry(start).or_default().entry(end).or_default() += 1;
                links
            },
        );

        Self { links }
    }

    fn grow(self, rules: &[Rule]) -> Self {
        let new_links = self
            .links
            .into_iter()
            .flat_map(|(start, ends)| {
                ends.into_iter().flat_map(move |(end, count)| {
                    if let Some(rule) = rules
                        .iter()
                        .find(|rule| rule.start == start && rule.end == end)
                    {
                        vec![(start, rule.insert, count), (rule.insert, end, count)]
                    } else {
                        vec![(start, end, count)]
                    }
                })
            })
            .fold(
                HashMap::<char, HashMap<char, usize>>::new(),
                |mut links, (start, end, count)| {
                    *links.entry(start).or_default().entry(end).or_default() += count;
                    links
                },
            );

        Self { links: new_links }
    }

    fn count_components(&self) -> anyhow::Result<HashMap<char, usize>> {
        let mut start_counts = self.links.iter().fold(
            HashMap::<char, usize>::new(),
            |mut counts, (start, ends)| {
                *counts.entry(*start).or_default() += ends.values().sum::<usize>();
                counts
            },
        );
        let end_counts = self.links.values().flat_map(|ends| ends.iter()).fold(
            HashMap::<char, usize>::new(),
            |mut counts, (end, count)| {
                *counts.entry(*end).or_default() += count;
                counts
            },
        );

        let (end, _count) = end_counts
            .iter()
            .find(|(end, count)| count > &&start_counts.get(*end).cloned().unwrap_or_default())
            .context("No end found")?;

        *start_counts.entry(*end).or_default() += 1;

        Ok(start_counts)
    }

    fn count_gap(&self) -> anyhow::Result<usize> {
        let component_counts = self.count_components()?;
        let max_count = component_counts.values().max().context("No max count")?;
        let min_count = component_counts.values().min().context("No min count")?;

        Ok(max_count - min_count)
    }
}

fn parse_input(input: &str) -> anyhow::Result<(Polymer, Vec<Rule>)> {
    let template = input.lines().next().context("Missing template")?;
    let polymer = Polymer::new(template);
    let rules = input.trim().lines().skip(2).map(Rule::parse).collect();

    Ok((polymer, rules))
}

pub struct Day14;

impl Aoc for Day14 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let (mut polymer, rules) = parse_input(input)?;

        for _ in 0..10 {
            polymer = polymer.grow(&rules[..]);
        }

        polymer.count_gap()
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let (mut polymer, rules) = parse_input(input)?;

        for _ in 0..40 {
            polymer = polymer.grow(&rules[..]);
        }

        polymer.count_gap()
    }
}
