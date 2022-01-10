use std::collections::{BinaryHeap, HashSet, LinkedList};

use anyhow::Context;

use crate::aoc::Aoc;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Cave {
    risks: Vec<Vec<usize>>,
}

impl Cave {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let risks = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|risk| risk.to_string().parse().context("Invalid risk"))
                    .collect::<anyhow::Result<_>>()
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(Cave { risks })
    }

    fn get_risk(&self, point: &Point) -> usize {
        self.risks[point.y][point.x]
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];

        if point.x > 0 {
            neighbors.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.x < self.risks[0].len() - 1 {
            neighbors.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y > 0 {
            neighbors.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.y < self.risks.len() - 1 {
            neighbors.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }

        neighbors
    }

    fn lowest_risk_amount(&self) -> anyhow::Result<usize> {
        let start = Point { x: 0, y: 0 };
        let goal = Point {
            x: self.risks[0].len() - 1,
            y: self.risks.len() - 1,
        };

        Ok(0)

        // let open_list: BinaryHeap<
        // let mut open_list: BinaryHeap< = Some(start).into_iter().collect();
        // let mut closed_list = HashSet::new();

        // loop {
        // let point = open_list.re
        // }

        //    make an openlist containing only the starting node
        //    make an empty closed list
        //    while (the destination node has not been reached):
        //        consider the node with the lowest f score in the open list
        //        if (this node is our destination node) :
        //            we are finished
        //        if not:
        //            put the current node in the closed list and look at all of its neighbors
        //            for (each neighbor of the current node):
        //                if (neighbor has lower g value than current and is in the closed list) :
        //                    replace the neighbor with the new, lower, g value
        //                    current node is now the neighbor's parent
        //                else if (current g value is lower and this neighbor is in the open list ) :
        //                    replace the neighbor with the new, lower, g value
        //                    change the neighbor's parent to our current node

        //                else if this neighbor is not in both lists:
        //                    add it to the open list and set its g    }
    }
}

pub struct Day15;

impl Aoc for Day15 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let cave = Cave::parse(input)?;

        cave.lowest_risk_amount()
    }

    fn part2(&self, _input: &str) -> anyhow::Result<usize> {
        unimplemented!()
    }
}
