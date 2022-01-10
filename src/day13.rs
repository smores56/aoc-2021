use std::collections::HashSet;

use anyhow::Context;

use crate::aoc::Aoc;

struct Paper {
    points: HashSet<Point>,
}

impl Paper {
    fn new(points: HashSet<Point>) -> Self {
        Self { points }
    }

    fn fold(&mut self, instruction: &Instruction) {
        let folded_points: Vec<Point> = if instruction.axis == Axis::X {
            self.points
                .drain_filter(|point| point.x > instruction.line)
                .map(|point| Point {
                    x: 2 * instruction.line - point.x,
                    y: point.y,
                })
                .collect()
        } else {
            self.points
                .drain_filter(|point| point.y > instruction.line)
                .map(|point| Point {
                    x: point.x,
                    y: 2 * instruction.line - point.y,
                })
                .collect()
        };

        self.points.extend(folded_points);
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(point: &str) -> anyhow::Result<Self> {
        let (x, y) = point
            .split_once(",")
            .context("Couldn't split coordinates")?;

        Ok(Self {
            x: x.parse().context("Failed to parse x coordinate")?,
            y: y.parse().context("Failed to parse y coordinate")?,
        })
    }
}

struct Instruction {
    line: usize,
    axis: Axis,
}

impl Instruction {
    fn parse(instruction: &str) -> anyhow::Result<Self> {
        let (start, line) = instruction.split_once("=").context("Missing equals")?;
        let axis = match start.chars().last() {
            Some('x') => Axis::X,
            Some('y') => Axis::Y,
            _ => anyhow::bail!("Invalid axis"),
        };

        Ok(Self {
            line: line.parse().context("Invalid line")?,
            axis,
        })
    }
}

#[derive(PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

fn parse_input(input: &str) -> anyhow::Result<(Paper, Vec<Instruction>)> {
    let (points, instructions) = input
        .split_once("\n\n")
        .context("Couldn't separate points from instructions")?;
    let points: HashSet<Point> = points
        .lines()
        .map(Point::parse)
        .collect::<anyhow::Result<_>>()?;
    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(Instruction::parse)
        .collect::<anyhow::Result<_>>()?;

    Ok((Paper::new(points), instructions))
}

pub struct Day13;

impl Aoc for Day13 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let (mut paper, instructions) = parse_input(input)?;

        instructions
            .into_iter()
            .take(1)
            .for_each(|instruction| paper.fold(&instruction));

        Ok(paper.points.len())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let (mut paper, instructions) = parse_input(input)?;

        instructions
            .into_iter()
            .for_each(|instruction| paper.fold(&instruction));

        let (max_x, max_y) = paper.points.iter().fold((0, 0), |(max_x, max_y), point| {
            (std::cmp::max(max_x, point.x), std::cmp::max(max_y, point.y))
        });

        for y in 0..=max_y {
            println!(
                "{}",
                (0..=max_x)
                    .map(|x| {
                        if paper.points.contains(&Point { x, y }) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            );
        }

        Ok(0)
    }
}
