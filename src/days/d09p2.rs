use std::{collections::HashSet, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<u64>> {
    let input = BufferedInput::parse_args("Day 9: Encoding Error - Part 2")?;

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

fn find_contiguous_sum(partials: &[u64], sum: u64) -> Option<(usize, usize)> {
    let (mut l, mut r) = (0, 2);
    let size = partials.len();

    while r < size && l < r {
        let (a, b) = (partials[l], partials[r]);
        let slice_sum = b - a;

        if slice_sum == sum {
            return Some((l, r));
        }

        if slice_sum > sum {
            l += 1;
        } else {
            r += 1;
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    let encrypted = parse_input()?;

    let searched = encrypted
        .windows(26)
        .find_map(|w| {
            if !is_valid_window(w) {
                w.last().copied()
            } else {
                None
            }
        })
        .unwrap();

    let partial_sums: Vec<_> = itertools::chain(std::iter::once(&0), &encrypted)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();

    let (index_a, index_b) = find_contiguous_sum(&partial_sums, searched).unwrap();

    let (min, max) = encrypted[index_a..index_b]
        .iter()
        .minmax()
        .into_option()
        .unwrap();

    println!("{}", min + max);

    Ok(())
}
