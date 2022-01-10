use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, pair};
use nom::IResult;

use crate::aoc::Aoc;

struct SnailfishNumber {
    left: SnailfishNumberType,
    right: SnailfishNumberType,
}

enum Side {
    Left,
    Right,
}

enum SnailfishNumberType {
    Literal(usize),
    Nested(Box<SnailfishNumber>),
}

impl SnailfishNumber {
    pub fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    pub fn reduce(self) -> Self {
        3 * self.lef1
    }

    // pub fn reduce_inner(self, level: usize) -> Self {
    //     if level >= 4 {

    //     } else {

    //     }
    //     self
    // }
}

impl SnailfishNumberType {
    pub fn magnitude(&self) -> usize {
        match self {
            SnailfishNumberType::Literal(value) => *value,
            SnailfishNumberType::Nested(number) => number.magnitude(),
        }
    }
}

impl std::ops::Add<Self> for SnailfishNumber {
    type Output = Self;

    fn add(self, other: SnailfishNumber) -> Self::Output {
        Self {
            left: SnailfishNumberType::Nested(Box::new(self)),
            right: SnailfishNumberType::Nested(Box::new(other)),
        }
        .reduce()
    }
}

fn parse_snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
    delimited(
        tag("["),
        map(
            pair(parse_snailfish_number_type, parse_snailfish_number_type),
            |(left, right)| SnailfishNumber { left, right },
        ),
        tag("]"),
    )(input)
}

fn parse_snailfish_number_type(input: &str) -> IResult<&str, SnailfishNumberType> {
    alt((
        map(parse_snailfish_number, |num| {
            SnailfishNumberType::Nested(Box::new(num))
        }),
        map_res(digit1, |digits: &str| {
            digits.parse().map(SnailfishNumberType::Literal)
        }),
    ))(input)
}

pub struct Day18;

impl Aoc for Day18 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let mut numbers: Vec<SnailfishNumber> = input
            .trim()
            .lines()
            .map(|line| {
                let (_rest, number) = parse_snailfish_number(line)
                    .map_err(|err| anyhow::anyhow!("Invalid number: {}", err))?;
                Ok(number)
            })
            .collect::<anyhow::Result<_>>()?;

        let mut base_number = numbers.remove(0);
        for number in numbers {
            base_number = base_number + number;
        }

        Ok(base_number.magnitude())
    }

    fn part2(&self, _input: &str) -> anyhow::Result<usize> {
        unimplemented!()
    }
}
