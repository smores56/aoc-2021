#![feature(path_try_exists, drain_filter)]

use chrono::{Datelike, Local};
use clap::Parser;

mod aoc;
mod input;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let args = Args::parse();
    let day = args.day.unwrap_or_else(|| Local::now().day() as usize);

    let day_impl = aoc::get_day(day)?;
    let input_for_day = input::get_input(day).await?;

    println!("Running part 1...");
    let before_part1 = Local::now();
    let part1 = day_impl.part1(&input_for_day)?;
    let after_part1 = Local::now();
    println!(
        "Part 1: {} (in {} seconds)",
        part1,
        (after_part1 - before_part1).num_milliseconds() as f64 / 1000.0,
    );

    println!("Running part 2...");
    let before_part2 = Local::now();
    let part2 = day_impl.part2(&input_for_day)?;
    let after_part2 = Local::now();
    println!(
        "Part 2: {} (in {} seconds)",
        part2,
        (after_part2 - before_part2).num_milliseconds() as f64 / 1000.0,
    );

    Ok(())
}

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long)]
    day: Option<usize>,
}
