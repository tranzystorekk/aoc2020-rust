use std::{collections::HashMap, io::Read};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<(Rules, Vec<Vec<u8>>)> {
    let mut input = BufferedInput::parse_args("Day 19: Monster Messages - Part 2")?;
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

fn check_matches(rules: &Rules, message: &[u8]) -> bool {
    let size = message.len();
    let mut stack = vec![(0, vec![0])];

    while let Some((index, mut current)) = stack.pop() {
        let id = current.pop().unwrap();
        let alternatives = &rules[&id];

        for rule in alternatives {
            match rule {
                Rule::Compound(ids) => {
                    let mut next = current.clone();
                    let elems = itertools::rev(ids);
                    next.extend(elems);

                    stack.push((index, next));
                }
                &Rule::Symbol(byte) => {
                    if byte != message[index] {
                        break;
                    }

                    let new_index = index + 1;
                    let is_at_end = new_index == size;
                    let no_more_rules = current.is_empty();

                    if is_at_end && no_more_rules {
                        return true;
                    }

                    if !is_at_end && !no_more_rules {
                        stack.push((new_index, current));
                    }

                    break; // a symbol rule contains only that symbol
                }
            }
        }
    }

    false
}

fn main() -> std::io::Result<()> {
    let (mut rules, messages) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let rule_8 = vec![Rule::Compound(vec![42]), Rule::Compound(vec![42, 8])];
        let rule_11 = vec![
            Rule::Compound(vec![42, 31]),
            Rule::Compound(vec![42, 11, 31]),
        ];

        rules.insert(8, rule_8);
        rules.insert(11, rule_11);

        messages
            .into_iter()
            .filter(|m| check_matches(&rules, m))
            .count()
    });

    Ok(())
}
