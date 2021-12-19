use crate::aoc::Aoc;
use anyhow::Context;

struct Board {
    squares: Vec<Vec<Square>>,
}

struct Square {
    value: usize,
    seen: bool,
}

impl Board {
    pub fn parse(rows: &[&str]) -> anyhow::Result<Board> {
        let squares = rows
            .iter()
            .map(|line| {
                line.trim()
                    .split_ascii_whitespace()
                    .map(|num| {
                        let value = num.parse().context("Invalid square")?;
                        Ok(Square { value, seen: false })
                    })
                    .collect::<anyhow::Result<Vec<Square>>>()
            })
            .collect::<anyhow::Result<Vec<Vec<Square>>>>()?;

        Ok(Board { squares })
    }

    fn visit(&mut self, num: usize) {
        for row in &mut self.squares {
            for square in row {
                if square.value == num {
                    square.seen = true;
                    return;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        if self
            .squares
            .iter()
            .any(|row| row.iter().all(|square| square.seen))
        {
            return true;
        }

        let row_length = self.squares[0].len();
        if (0..row_length).any(|column_index| self.squares.iter().all(|row| row[column_index].seen))
        {
            return true;
        }

        return false;
    }

    fn score(&self, drawing: usize) -> usize {
        let sum_of_unseen: usize = self
            .squares
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|square| {
                if !square.seen {
                    Some(square.value)
                } else {
                    None
                }
            })
            .sum();

        sum_of_unseen * drawing
    }
}

fn parse_bingo(input: &str) -> anyhow::Result<(Vec<usize>, Vec<Board>)> {
    let mut lines = input.lines().peekable();
    let drawings = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("No drawings for bingo"))?
        .split(",")
        .map(|number| number.parse().context("Invalid drawing"))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    let lines = lines.collect::<Vec<_>>();
    let boards = lines
        .chunks(6)
        .map(|slice| Board::parse(&slice[1..]))
        .collect::<anyhow::Result<Vec<Board>>>()?;

    Ok((drawings, boards))
}

pub struct Day4;

impl Aoc for Day4 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let (drawings, mut boards) = parse_bingo(input)?;
        let mut drawing_index = 0;

        loop {
            let drawing = drawings[drawing_index];
            boards.iter_mut().for_each(|board| board.visit(drawing));

            if let Some(winner) = boards.iter().find(|board| board.has_won()) {
                return Ok(winner.score(drawing));
            }

            drawing_index += 1;
        }
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let (drawings, mut boards) = parse_bingo(input)?;
        let mut drawing_index = 0;
        let mut winning_score = 0;

        while boards.len() > 0 && drawing_index < drawings.len() {
            let drawing = drawings[drawing_index];
            boards.iter_mut().for_each(|board| board.visit(drawing));

            let mut winners = boards.drain_filter(|board| board.has_won());
            if let Some(winner) = winners.next() {
                winning_score = winner.score(drawing);
            }

            drawing_index += 1;
        }

        Ok(winning_score)
    }
}
