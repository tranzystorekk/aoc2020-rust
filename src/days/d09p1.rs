use std::{collections::HashSet, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<u64>> {
    let input = BufferedInput::parse_args("Day 9: Encoding Error - Part 1")?;

    input
        .lines()
        .map_results(|line| line.parse().unwrap())
        .collect()
}

fn is_valid_window(window: &[u64]) -> bool {
    let (preamble, validated) = match window {
        [preamble @ .., v] => (preamble, v),
        _ => unreachable!(),
    };
    let mut remainders = HashSet::new();

    preamble.iter().any(|&v| {
        if remainders.contains(&v) {
            return true;
        }

        !remainders.insert(validated - v)
    })
}

fn main() -> std::io::Result<()> {
    let encrypted = parse_input()?;

    let result = encrypted
        .windows(26)
        .find_map(|w| {
            if !is_valid_window(w) {
                w.last().copied()
            } else {
                None
            }
        })
        .unwrap();

    println!("{}", result);

    Ok(())
}
