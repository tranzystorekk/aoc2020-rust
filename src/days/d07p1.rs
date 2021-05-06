use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;

type Regulations = HashMap<String, Vec<String>>;

fn parse_input() -> std::io::Result<Regulations> {
    let input = BufferedInput::parse_args("Day 7: Handy Haversacks - Part 1")?;

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
                    Some(result_color)
                })
                .collect();

            (outer, contents)
        })
        .collect()
}

fn can_contain_shiny_gold(regulations: &Regulations, outer: &str) -> bool {
    let mut searchspace = vec![outer];

    while let Some(container) = searchspace.pop() {
        if container == "shiny gold" {
            return true;
        }

        let inner = &regulations[container];
        let search_candidates = inner.iter().map(String::as_str);

        searchspace.extend(search_candidates);
    }

    false
}

fn main() -> std::io::Result<()> {
    let regulations = parse_input()?;

    let (elapsed, n_accepted_containers) = elapsed::measure_time(|| {
        regulations
            .keys()
            .filter(|&c| c != "shiny gold")
            .filter(|c| can_contain_shiny_gold(&regulations, c))
            .count()
    });

    eprintln!("{}", elapsed);
    println!("{}", n_accepted_containers);

    Ok(())
}
