use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<(HashSet<String>, HashSet<String>)>> {
    let input = BufferedInput::parse_args("Day 21: Allergen Assessment - Part 2")?;

    input
        .lines()
        .map_results(|line| {
            let (ings, algs) = line
                .strip_suffix(')')
                .unwrap()
                .split(" (contains ")
                .collect_tuple()
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

        // find possible carrier ingredients for each allergen
        let mut matches = HashMap::with_capacity(allergens.len());

        for &alg in &allergens {
            let possible_carriers = foods
                .iter()
                .filter_map(|(ings, algs)| Some(ings).filter(|_| algs.contains(alg)))
                .fold(HashSet::new(), |acc, ings| {
                    if acc.is_empty() {
                        ings.clone()
                    } else {
                        &acc & ings
                    }
                });

            matches.insert(alg, possible_carriers);
        }

        // incrementally scan for carriers that perfectly match an allergen
        let mut mappings = Vec::with_capacity(matches.len());

        while !matches.is_empty() {
            let matching_allergen = matches
                .iter()
                .find_map(|(&alg, ings)| Some(alg).filter(|_| ings.len() == 1))
                .unwrap();

            let matching_ingredient = matches
                .remove(matching_allergen)
                .and_then(|ings| ings.into_iter().next())
                .unwrap();

            for ings in matches.values_mut() {
                ings.remove(&matching_ingredient);
            }

            mappings.push((matching_allergen, matching_ingredient));
        }

        mappings.sort_unstable_by(|(alg_a, _), (alg_b, _)| alg_a.cmp(alg_b));
        mappings.into_iter().map(|(_, ing)| ing).join(",")
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
