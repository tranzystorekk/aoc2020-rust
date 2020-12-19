use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<(Rules, HashSet<Vec<u8>>)> {
    let mut input = BufferedInput::parse_args("Day 19: Monster Messages - Part 1")?;
    let mut file = String::new();
    input.read_to_string(&mut file)?;

    let mut split = file.split("\n\n");

    let rules = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (id, rule_str) = scan_fmt!(line, "{d}: {[ \"|0-9a-z]}", _, String).unwrap();

            let alternatives = rule_str
                .split(" | ")
                .map(|alt| {
                    if alt.starts_with('"') {
                        let sym = alt.as_bytes()[1];
                        Rule::Symbol(sym)
                    } else {
                        let seq = alt.split_whitespace().map(|n| n.parse().unwrap()).collect();
                        Rule::Compound(seq)
                    }
                })
                .collect();

            (id, alternatives)
        })
        .collect();

    let messages = split.next().unwrap().trim().lines().map_into().collect();

    Ok((rules, messages))
}

type Rules = HashMap<i32, Vec<Rule>>;

enum Rule {
    Compound(Vec<i32>),
    Symbol(u8),
}

fn count_matches(rules: &Rules, messages: &HashSet<Vec<u8>>) -> usize {
    let mut stack = vec![(vec![], vec![0])];
    let mut n_matches = 0;

    while let Some((mut matched, mut current)) = stack.pop() {
        if let Some(id) = current.pop() {
            let alternatives = &rules[&id];

            for rule in alternatives {
                match rule {
                    Rule::Compound(ids) => {
                        let mut cl = current.clone();
                        let elems = ids.iter().copied().rev();
                        cl.extend(elems);

                        stack.push((matched.clone(), cl));
                    }
                    &Rule::Symbol(byte) => {
                        matched.push(byte);
                        if current.is_empty() {
                            n_matches += messages.contains(&matched) as usize;
                        } else {
                            stack.push((matched, current));
                        }
                        break; // a symbol rule contains only that symbol
                    }
                }
            }
        }
    }

    n_matches
}

fn main() -> std::io::Result<()> {
    let (rules, messages) = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| count_matches(&rules, &messages));

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
