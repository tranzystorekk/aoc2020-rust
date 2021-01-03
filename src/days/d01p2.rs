use aoc_utils::BufferedInput;
use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 1: Report Repair - Part 2")?;

    input
        .lines()
        .map_ok(|line| line.parse().unwrap())
        .collect()
}

fn try_sum_two(first: i32, records: &[i32]) -> Option<(i32, i32, i32)> {
    let mut remainders = HashSet::new();
    let remaining_sum = 2020 - first;

    records
        .iter()
        .filter(|&&el| el != first)
        .find(|&&record| {
            if remainders.contains(&record) {
                return true;
            }

            let other = remaining_sum - record;
            !remainders.insert(other)
        })
        .map(|&n| (first, remaining_sum - n, n))
}

fn main() -> std::io::Result<()> {
    let records = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let (first, second, third) = records
            .iter()
            .find_map(|&first| try_sum_two(first, &records))
            .unwrap();

        first * second * third
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
