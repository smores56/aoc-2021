pub trait Aoc {
    fn part1(&self, input: &str) -> anyhow::Result<usize>;
    fn part2(&self, input: &str) -> anyhow::Result<usize>;
}

pub fn get_day(day: usize) -> anyhow::Result<Box<dyn Aoc>> {
    match day {
        1 => Ok(Box::new(crate::day1::Day1)),
        2 => Ok(Box::new(crate::day2::Day2)),
        3 => Ok(Box::new(crate::day3::Day3)),
        4 => Ok(Box::new(crate::day4::Day4)),
        5 => Ok(Box::new(crate::day5::Day5)),
        6 => Ok(Box::new(crate::day6::Day6)),
        7 => Ok(Box::new(crate::day7::Day7)),
        8 => Ok(Box::new(crate::day8::Day8)),
        9 => Ok(Box::new(crate::day9::Day9)),
        10 => Ok(Box::new(crate::day10::Day10)),
        11 => Ok(Box::new(crate::day11::Day11)),
        12 => Ok(Box::new(crate::day12::Day12)),
        13 => Ok(Box::new(crate::day13::Day13)),
        14 => Ok(Box::new(crate::day14::Day14)),
        15 => Ok(Box::new(crate::day15::Day15)),
        16 => Ok(Box::new(crate::day16::Day16)),
        17 => Ok(Box::new(crate::day17::Day17)),
        18 => Ok(Box::new(crate::day18::Day18)),
        19 => Ok(Box::new(crate::day19::Day19)),
        20 => Ok(Box::new(crate::day20::Day20)),
        21 => Ok(Box::new(crate::day21::Day21)),
        22 => Ok(Box::new(crate::day22::Day22)),
        23 => Ok(Box::new(crate::day23::Day23)),
        24 => Ok(Box::new(crate::day24::Day24)),
        25 => Ok(Box::new(crate::day25::Day25)),
        _ => Err(anyhow::anyhow!("Unsupported day: {}", day)),
    }
}
