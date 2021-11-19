use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;

type Regulations = HashMap<String, Vec<(String, i32)>>;

fn parse_input() -> std::io::Result<Regulations> {
    let input = BufferedInput::parse_args("Day 7: Handy Haversacks - Part 2")?;

    input
        .lines()
        .map_ok(|line| {
            let (outer, contents) = line.split_once(" contain ").unwrap();
            let outer = outer.split_whitespace().take(2).join(" ");

            let contents = contents
                .split(", ")
                .filter_map(|descr| {
                    let (n, ca, cb) = descr.split_whitespace().next_tuple().unwrap();

                    if n == "no" {
                        return None;
                    }

                    let result_color = format!("{} {}", ca, cb);
                    let number = n.parse().unwrap();
                    Some((result_color, number))
                })
                .collect();

            (outer, contents)
        })
        .collect()
}

fn count_inner_bags(regulations: &Regulations, outer: &str) -> i32 {
    let mut searchspace: Vec<_> = regulations[outer]
        .iter()
        .map(|(c, n)| (c.as_str(), *n))
        .collect();
    let mut result = 0;

    while let Some((container, n)) = searchspace.pop() {
        result += n;

        let inner = &regulations[container];
        let search_candidates = inner
            .iter()
            .map(|(color, num_regulated)| (color.as_str(), n * num_regulated));

        searchspace.extend(search_candidates);
    }

    result
}

fn main() -> std::io::Result<()> {
    let regulations = parse_input()?;

    aoc_utils::measure_and_print(|| count_inner_bags(&regulations, "shiny gold"));

    Ok(())
}
