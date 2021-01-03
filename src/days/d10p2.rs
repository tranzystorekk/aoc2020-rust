use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;

type Connections = HashMap<i32, Vec<i32>>;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 10: Adapter Array - Part 2")?;

    input
        .lines()
        .map_ok(|line| line.parse().unwrap())
        .collect()
}

fn prepare_connections(sorted: &[i32]) -> Connections {
    sorted
        .iter()
        .enumerate()
        .map(|(i, &current)| {
            let start_index = i + 1;
            let slice = &sorted[start_index..];
            let valid_connections = slice
                .iter()
                .copied()
                .take_while(|&v| v <= current + 3)
                .collect();

            (current, valid_connections)
        })
        .collect()
}

fn traverse_connections(sorted: &[i32], connections: &Connections) -> u64 {
    // store counts of distinct paths from key to target
    let mut dynamic_counts = {
        let mut res = HashMap::with_capacity(sorted.len());
        res.insert(sorted.last().copied().unwrap(), 1);
        res
    };

    // dynamic programming: going back down the topologically sorted joltage values
    // and summing up all paths to target
    for &el in sorted.iter().rev() {
        let current: u64 = connections
            .get(&el)
            .into_iter()
            .flatten()
            .map(|v| dynamic_counts[v])
            .sum();
        let count = dynamic_counts.entry(el).or_default();
        *count += current;
    }

    dynamic_counts[&0]
}

fn main() -> std::io::Result<()> {
    let mut adapters = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        adapters.push(0);
        adapters.sort_unstable();
        adapters.push(adapters.last().unwrap() + 3);

        let connections = prepare_connections(&adapters);

        traverse_connections(&adapters, &connections)
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
