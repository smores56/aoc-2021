use std::collections::HashMap;

use anyhow::Context;

use crate::aoc::Aoc;

#[derive(Debug)]
struct CaveSystem<'s> {
    edges: HashMap<&'s str, Vec<&'s str>>,
}

#[derive(Clone, Debug)]
struct Path<'s> {
    caves: Vec<&'s str>,
    has_repeat: bool,
}

impl<'s> CaveSystem<'s> {
    fn parse(input: &'s str) -> anyhow::Result<Self> {
        let adjacencies: Vec<(&str, &str)> = input
            .trim()
            .lines()
            .map(|line| line.split_once("-").context("Invalid node pair"))
            .collect::<anyhow::Result<_>>()?;

        let mut edges = HashMap::<&str, Vec<&str>>::new();
        for (from, to) in adjacencies {
            edges.entry(from).or_default().push(to);
            edges.entry(to).or_default().push(from);
        }

        Ok(Self { edges })
    }

    fn visible_caves<'cs: 's>(&'cs self, from: &'s str) -> impl Iterator<Item = &'s str> + 'cs {
        self.edges
            .get(from)
            .into_iter()
            .flat_map(|caves| caves.iter().cloned())
            .filter(|cave| cave != &"start")
    }

    fn all_paths<'cs: 's>(&'cs self, path: Path<'s>, allow_repeat: bool) -> Vec<Path<'s>> {
        match path.caves.last().cloned() {
            None => self.all_paths(Path::single("start"), allow_repeat),
            Some(last_cave) if last_cave != "end" => self
                .visible_caves(last_cave)
                .flat_map(|visible_cave| {
                    let lowercase = visible_cave.chars().all(char::is_lowercase);
                    let seen = path.caves.contains(&visible_cave);

                    if lowercase && seen && !(allow_repeat && !path.has_repeat) {
                        vec![]
                    } else {
                        self.all_paths(path.join(visible_cave), allow_repeat)
                    }
                })
                .collect(),
            _ => vec![path],
        }
    }
}

impl<'s> Path<'s> {
    fn empty() -> Path<'s> {
        Path {
            caves: vec![],
            has_repeat: false,
        }
    }

    fn single(cave: &'s str) -> Path<'s> {
        Path {
            caves: vec![cave],
            has_repeat: false,
        }
    }

    fn join(&self, cave: &'s str) -> Path<'s> {
        let has_repeat = if self.has_repeat {
            true
        } else {
            let lowercase = cave.chars().all(char::is_lowercase);
            let other_found = self.caves.iter().any(|c| c == &cave);
            lowercase && other_found
        };

        Path {
            caves: self.caves.iter().cloned().chain(Some(cave)).collect(),
            has_repeat,
        }
    }
}

pub struct Day12;

impl Aoc for Day12 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let system = CaveSystem::parse(input)?;
        let paths = system.all_paths(Path::empty(), false);

        Ok(paths.len())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let system = CaveSystem::parse(input)?;
        let paths = system.all_paths(Path::empty(), true);

        Ok(paths.len())
    }
}
