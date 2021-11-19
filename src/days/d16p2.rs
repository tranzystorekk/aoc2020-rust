use std::{collections::HashSet, io::Read};

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<(Vec<Req>, Vec<Ticket>, Ticket)> {
    let mut input = BufferedInput::parse_args("Day 16: Ticket Translation - Part 2")?;
    let mut file = String::new();
    input.read_to_string(&mut file)?;

    let mut parts = file.split("\n\n");

    let reqs = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            let (name, ranges) = line.split_once(": ").unwrap();
            let (a, b) = ranges.split_once(" or ").unwrap();

            let a_range = a
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let b_range = b
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            if name.starts_with("departure") {
                Req::Depart(a_range, b_range)
            } else {
                Req::Other(a_range, b_range)
            }
        })
        .collect();

    let my_ticket = parts
        .next()
        .unwrap()
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let tickets = parts
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Ok((reqs, tickets, my_ticket))
}

type Range = (u64, u64);
type Ticket = Vec<u64>;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Req {
    Depart(Range, Range),
    Other(Range, Range),
}

impl Req {
    pub fn to_ranges(&self) -> (Range, Range) {
        match *self {
            Req::Depart(a, b) | Req::Other(a, b) => (a, b),
        }
    }

    pub fn is_depart(&self) -> bool {
        matches!(self, Req::Depart(_, _))
    }

    pub fn is_valid(&self, value: u64) -> bool {
        let ((a_from, a_to), (b_from, b_to)) = self.to_ranges();

        let a_valid = || value >= a_from && value <= a_to;
        let b_valid = || value >= b_from && value <= b_to;

        a_valid() || b_valid()
    }

    pub fn check_column(&self, column: &[u64]) -> bool {
        column.iter().all(|&val| self.is_valid(val))
    }
}

fn transpose(table: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let inner_size = table[0].len();
    let mut result = Vec::with_capacity(inner_size);

    for i in 0..inner_size {
        let column = table.iter().map(|row| row[i]).collect();
        result.push(column);
    }

    result
}

fn main() -> std::io::Result<()> {
    let (reqs, tickets, my_ticket) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let valid = tickets
            .into_iter()
            .filter(|ticket| ticket.iter().all(|&v| reqs.iter().any(|r| r.is_valid(v))))
            .collect();
        let columns = transpose(valid);

        let mut positions_left: HashSet<usize> = (0..my_ticket.len()).collect();
        let mut fields_left: HashSet<Req> = reqs.into_iter().collect();
        let mut result = 1;
        let mut n_departs = 0;

        while n_departs < 6 {
            let (pos, field) = fields_left
                .iter()
                .copied()
                .find_map(|req| {
                    positions_left
                        .iter()
                        .copied()
                        .filter(|&pos| req.check_column(&columns[pos]))
                        .exactly_one()
                        .ok()
                        .map(|pos| (pos, req))
                })
                .unwrap();

            positions_left.remove(&pos);
            fields_left.remove(&field);

            if field.is_depart() {
                result *= my_ticket[pos];
                n_departs += 1;
            }
        }

        result
    });

    Ok(())
}
