use aoc_utils::BufferedInput;
use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 1: Report Repair - Part 1")?;

    input
        .lines()
        .map_results(|line| line.parse().unwrap())
        .collect()
}

fn main() -> std::io::Result<()> {
    let records = parse_input()?;
    let mut remainders = HashSet::new();

    let second = records
        .into_iter()
        .find(|&entry| {
            if remainders.contains(&entry) {
                return true;
            }

            let other = 2020 - entry;
            !remainders.insert(other)
        })
        .unwrap();
    
    let first = 2020 - second;

    println!("{}", first * second);

    Ok(())
}
