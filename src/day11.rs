use std::collections::{HashSet, VecDeque};

use anyhow::Context;

use crate::aoc::Aoc;

#[derive(Debug)]
struct OctopusField {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl OctopusField {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let grid: Vec<Vec<usize>> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| digit.to_string().parse().context("invalid digit"))
                    .collect::<anyhow::Result<_>>()
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(OctopusField {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        })
    }

    fn get_energy(&self, pos: &Position) -> usize {
        self.grid[pos.y][pos.x]
    }

    fn set_energy(&mut self, pos: &Position, energy: usize) {
        self.grid[pos.y][pos.x] = energy;
    }

    fn increase_energy(&mut self, pos: &Position, step: usize) -> usize {
        let new_energy = self.grid[pos.y][pos.x] + step;
        self.grid[pos.y][pos.x] = new_energy;

        new_energy
    }

    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        let left_open = pos.x > 0;
        let right_open = pos.x < self.width - 1;
        let top_open = pos.y > 0;
        let bottom_open = pos.y < self.height - 1;
        let mut neighbors = vec![];

        if top_open {
            neighbors.push(Position {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        if right_open && top_open {
            neighbors.push(Position {
                x: pos.x + 1,
                y: pos.y - 1,
            });
        }
        if right_open {
            neighbors.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if right_open && bottom_open {
            neighbors.push(Position {
                x: pos.x + 1,
                y: pos.y + 1,
            });
        }
        if bottom_open {
            neighbors.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        if left_open && bottom_open {
            neighbors.push(Position {
                x: pos.x - 1,
                y: pos.y + 1,
            });
        }
        if left_open {
            neighbors.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        if left_open && top_open {
            neighbors.push(Position {
                x: pos.x - 1,
                y: pos.y - 1,
            });
        }

        neighbors
    }

    fn all_positions(&self) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (0..width).flat_map(move |x| (0..height).map(move |y| Position { x, y }))
    }

    fn step(&mut self) -> usize {
        self.all_positions().for_each(|pos| {
            self.increase_energy(&pos, 1);
        });

        let mut flashing = self
            .all_positions()
            .filter(|pos| self.get_energy(pos) > 9)
            .collect::<Vec<_>>();
        let mut flashed = flashing.iter().cloned().collect::<HashSet<_>>();

        while let Some(current) = flashing.pop() {
            flashed.insert(current.clone());

            for neighbor in self.neighbors(&current) {
                let new_energy = self.increase_energy(&neighbor, 1);
                if new_energy > 9 && !flashed.contains(&neighbor) {
                    flashing.push(neighbor);
                }
            }
        }

        flashed.iter().for_each(|pos| self.set_energy(&pos, 0));

        flashed.len()
    }
}

impl std::fmt::Display for OctopusField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|energy| energy.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .intersperse("\n".to_owned())
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}", grid)
    }
}

pub struct Day11;

impl Aoc for Day11 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let mut field = OctopusField::parse(input)?;
        let mut total_flashes = 0;

        println!("{}\n", field);
        for _ in 0..10 {
            total_flashes += field.step();
            println!("{}\n", field);
        }

        Ok(total_flashes)
    }

    fn part2(&self, _input: &str) -> anyhow::Result<usize> {
        unimplemented!()
    }
}
