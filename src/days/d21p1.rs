use std::{collections::HashSet, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<(HashSet<String>, HashSet<String>)>> {
    let input = BufferedInput::parse_args("Day 21: Allergen Assessment - Part 1")?;

    input
        .lines()
        .map_ok(|line| {
            let (ings, algs) = line
                .strip_suffix(')')
                .unwrap()
                .split_once(" (contains ")
                .unwrap();

            (
                ings.split_whitespace().map_into().collect(),
                algs.split(", ").map_into().collect(),
            )
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let foods = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let allergens = foods.iter().fold(HashSet::new(), |mut acc, (_, algs)| {
            acc.extend(algs);
            acc
        });

        let mut carriers = HashSet::new();

        for &alg in &allergens {
            let possible_carriers = foods
                .iter()
                .filter_map(|(ings, algs)| algs.contains(alg).then(|| ings))
                .fold(HashSet::new(), |acc, ings| {
                    if acc.is_empty() {
                        ings.clone()
                    } else {
                        &acc & ings
                    }
                });

            carriers.extend(possible_carriers);
        }

        foods
            .into_iter()
            .flat_map(|(ingredients, _)| ingredients)
            .filter(|ing| !carriers.contains(ing))
            .count()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
