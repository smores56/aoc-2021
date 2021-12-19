use std::collections::HashSet;

use anyhow::Context;

use crate::aoc::Aoc;

struct Floor {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Floor {
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

        Ok(Floor {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        })
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn basin_centers(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &depth)| {
                    if self
                        .neighbors(x, y)
                        .iter()
                        .all(|&(nx, ny)| self.grid[ny][nx] > depth)
                    {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut neighbor_queue = vec![(x, y)];
        let mut visited = HashSet::new();

        while let Some(current_position) = neighbor_queue.pop() {
            let (current_x, current_y) = current_position;
            let current_depth = self.grid[current_y][current_x];
            visited.insert(current_position);

            neighbor_queue.extend(self.neighbors(current_x, current_y).iter().filter(
                |&&(nx, ny)| {
                    self.grid[ny][nx] > current_depth
                        && self.grid[ny][nx] < 9
                        && !visited.contains(&(nx, ny))
                },
            ));
        }

        visited.len()
    }
}

pub struct Day9;

impl Aoc for Day9 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let floor = Floor::parse(input)?;
        let basin_centers = floor.basin_centers();

        Ok(basin_centers
            .into_iter()
            .map(|(x, y)| floor.grid[y][x] + 1)
            .sum())
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let floor = Floor::parse(input)?;
        let basin_centers = floor.basin_centers();

        let mut basin_sizes = basin_centers
            .into_iter()
            .map(|(x, y)| floor.basin_size(x, y))
            .collect::<Vec<usize>>();
        basin_sizes.sort();

        Ok(basin_sizes
            .iter()
            .rev()
            .take(3)
            .fold(1, |product, size| product * size))
    }
}
