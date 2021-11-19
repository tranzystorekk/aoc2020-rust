use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 10: Adapter Array - Part 1")?;

    input.lines().map_ok(|line| line.parse().unwrap()).collect()
}

fn main() -> std::io::Result<()> {
    let mut adapters = parse_input()?;

    aoc_utils::measure_and_print(|| {
        adapters.sort_unstable();
        adapters.push(adapters.last().unwrap() + 3);

        let (ones, threes) = adapters
            .into_iter()
            .scan(0, |current, joltage| {
                let diff = joltage - *current;
                *current = joltage;
                Some(diff)
            })
            .fold((0, 0), |(ones, threes), diff| match diff {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            });

        ones * threes
    });

    Ok(())
}
