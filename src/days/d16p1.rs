use std::io::Read;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<(Requirements, Vec<Ticket>)> {
    let mut input = BufferedInput::parse_args("Day 16: Ticket Translation - Part 1")?;
    let mut file = String::new();
    input.read_to_string(&mut file)?;

    let mut parts = file.split("\n\n");

    let reqs = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            let ranges = line.split(": ").nth(1).unwrap();
            let (a, b) = ranges.split(" or ").collect_tuple().unwrap();

            let (a_from, a_to) = a
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (b_from, b_to) = b
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            vec![(a_from, a_to), (b_from, b_to)]
        })
        .flatten()
        .collect();

    let tickets = parts
        .nth(1)
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Ok((Requirements(reqs), tickets))
}

type Ticket = Vec<i32>;

struct Requirements(Vec<(i32, i32)>);

impl Requirements {
    pub fn error_rate(&self, ticket: &[i32]) -> i32 {
        ticket
            .iter()
            .filter(|&&val| self.0.iter().all(|&(from, to)| val < from || val > to))
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    let (reqs, tickets) = parse_input()?;

    let (elapsed, result): (_, i32) =
        elapsed::measure_time(|| tickets.iter().map(|ticket| reqs.error_rate(ticket)).sum());

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
