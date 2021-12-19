use crate::aoc::Aoc;
use anyhow::Context;
use std::str::FromStr;

struct Command {
    direction: Direction,
    distance: usize,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(command: &str) -> anyhow::Result<Self> {
        let mut part_iter = command.split(" ");
        let direction = Direction::from_str(
            part_iter
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing direction"))?,
        )?;
        let distance = part_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing distance"))?
            .parse()
            .context("Invalid distance")?;

        Ok(Command {
            direction,
            distance,
        })
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(direction: &str) -> anyhow::Result<Self> {
        match direction {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(anyhow::anyhow!("Invalid direction")),
        }
    }
}

pub struct Day2;

impl Aoc for Day2 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let commands = input
            .lines()
            .map(Command::from_str)
            .collect::<anyhow::Result<Vec<Command>>>()?;

        let mut position = 0;
        let mut depth = 0;

        for command in commands {
            match command.direction {
                Direction::Forward => {
                    position += command.distance;
                }
                Direction::Down => {
                    depth += command.distance;
                }
                Direction::Up => {
                    depth -= command.distance;
                }
            }
        }

        Ok(position * depth)
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let commands = input
            .lines()
            .map(Command::from_str)
            .collect::<anyhow::Result<Vec<Command>>>()?;

        let mut position = 0;
        let mut depth = 0;
        let mut aim = 0;

        for command in commands {
            match command.direction {
                Direction::Forward => {
                    position += command.distance;
                    depth += aim * command.distance;
                }
                Direction::Down => {
                    aim += command.distance;
                }
                Direction::Up => {
                    aim -= command.distance;
                }
            }
        }

        Ok(position * depth)
    }
}
