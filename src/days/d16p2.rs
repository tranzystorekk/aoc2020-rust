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
            let (name, ranges) = line.split(": ").collect_tuple().unwrap();
            let (a, b) = ranges.split(" or ").collect_tuple().unwrap();

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

type Ticket = Vec<u64>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Req {
    Depart((u64, u64), (u64, u64)),
    Other((u64, u64), (u64, u64)),
}

impl Req {
    pub fn to_ranges(&self) -> ((u64, u64), (u64, u64)) {
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

    pub fn check_matching(&self, tickets: &[Ticket], candidates: HashSet<usize>) -> Option<usize> {
        tickets
            .iter()
            .fold(candidates, |cands, ticket| {
                ticket
                    .iter()
                    .positions(|&val| self.is_valid(val))
                    .filter(|pos| cands.contains(pos))
                    .collect()
            })
            .iter()
            .copied()
            .exactly_one()
            .ok()
    }
}

fn main() -> std::io::Result<()> {
    let (reqs, mut tickets, my_ticket) = parse_input()?;

    let (elapsed, result): (_, u64) = elapsed::measure_time(|| {
        tickets.retain(|ticket| ticket.iter().all(|&v| reqs.iter().any(|r| r.is_valid(v))));

        let mut positions_left: HashSet<usize> = (0..my_ticket.len()).collect();
        let mut fields_left: HashSet<Req> = reqs.into_iter().collect();
        let mut result = 1;
        let mut n_departs = 0;

        while n_departs < 6 {
            let (pos, field) = fields_left
                .iter()
                .copied()
                .find_map(|req| {
                    req.check_matching(&tickets, positions_left.clone())
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

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
