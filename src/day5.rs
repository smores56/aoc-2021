use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::u32 as parse_u32, IResult};

use crate::aoc::Aoc;

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl LineSegment {
    fn bottom_and_top(&self) -> (usize, usize) {
        if self.start.y > self.end.y {
            (self.end.y, self.start.y)
        } else {
            (self.start.y, self.end.y)
        }
    }

    fn left_and_right(&self) -> (usize, usize) {
        if self.start.x > self.end.x {
            (self.end.x, self.start.x)
        } else {
            (self.start.x, self.end.x)
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_u32(input)?;

    Ok((
        input,
        Point {
            x: x as usize,
            y: y as usize,
        },
    ))
}

fn parse_line_segment(input: &str) -> IResult<&str, LineSegment> {
    let (input, start) = parse_point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, end) = parse_point(input)?;

    Ok((input, LineSegment { start, end }))
}

fn parse_line_segments(input: &str) -> anyhow::Result<Vec<LineSegment>> {
    input
        .lines()
        .map(|line| {
            parse_line_segment(line)
                .map(|(_rest, line_segment)| line_segment)
                .map_err(|err| anyhow::anyhow!("Failed to parse line segment: {}", err))
        })
        .collect()
}

fn points_for_line(line: &LineSegment) -> Box<dyn Iterator<Item = Point>> {
    let (bottom, top) = line.bottom_and_top();
    let (left, right) = line.left_and_right();

    if line.start.x == line.end.x {
        let x = line.start.x;
        Box::new((bottom..=top).map(move |y| Point { x, y }))
    } else if line.start.y == line.end.y {
        let y = line.start.y;
        Box::new((left..=right).map(move |x| Point { x, y }))
    } else if (line.start.x > line.end.x && line.start.y > line.end.y)
        || (line.start.x < line.end.x && line.start.y < line.end.y)
    {
        Box::new(
            (left..=right)
                .zip(bottom..=top)
                .map(|(x, y)| Point { x, y }),
        )
    } else {
        Box::new(
            (left..=right)
                .rev()
                .zip(bottom..=top)
                .map(|(x, y)| Point { x, y }),
        )
    }
}

pub struct Day5;

impl Aoc for Day5 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let lines = parse_line_segments(input)?;
        let mut grid: HashMap<Point, usize> = HashMap::new();

        lines
            .iter()
            .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
            .flat_map(points_for_line)
            .for_each(|point| {
                *grid.entry(point).or_default() += 1;
            });

        let intersection_count = grid.values().filter(|&&count| count > 1).count();

        Ok(intersection_count)
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let lines = parse_line_segments(input)?;
        let mut grid: HashMap<Point, usize> = HashMap::new();

        lines.iter().flat_map(points_for_line).for_each(|point| {
            *grid.entry(point).or_default() += 1;
        });

        let intersection_count = grid.values().filter(|&&count| count > 1).count();

        Ok(intersection_count)
    }
}
