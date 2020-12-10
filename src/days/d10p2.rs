use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use maplit::hashmap;

type Connections = HashMap<i32, Vec<i32>>;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 10: Adapter Array - Part 2")?;

    input
        .lines()
        .map_results(|line| line.parse().unwrap())
        .collect()
}

fn prepare_connections(sorted: &[i32]) -> Connections {
    itertools::chain(std::iter::once(&0), sorted)
        .enumerate()
        .map(|(i, &current)| {
            let slice = &sorted[i..];
            let valid_connections = slice
                .iter()
                .copied()
                .take_while(|&v| v <= current + 3)
                .collect();

            (current, valid_connections)
        })
        .collect()
}

fn traverse_connections(sorted: &[i32], connections: &Connections, target: i32) -> u64 {
    let mut dynamic_counts = hashmap! { target => 1 };
    let with_outlet = itertools::chain(std::iter::once(&0), sorted);

    for &el in with_outlet.rev() {
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
        adapters.sort_unstable();
        adapters.push(adapters.last().unwrap() + 3);

        let connections = prepare_connections(&adapters);
        let socket = adapters.last().copied().unwrap();

        traverse_connections(&adapters, &connections, socket)
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
