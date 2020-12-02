use aoc_utils::BufferedInput;
use itertools::Itertools;

type Policied = (usize, usize, char, String);

fn parse_input() -> std::io::Result<Vec<Policied>> {
    let input = BufferedInput::parse_args("Day 2: Password Philosophy - Part 1")?;

    let result = input
        .unwrapped_lines()
        .map(|line| {
            let words = line.split_whitespace();
            let (range, ch, pass) = words.collect_tuple().unwrap();

            let (min, max) = range
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let letter = ch.chars().next().unwrap();

            (min, max, letter, pass.into())
        })
        .collect();

    Ok(result)
}

fn verify_policy(min: usize, max: usize, l: char, password: &str) -> bool {
    let occurrences = password.chars().filter(|&c| c == l).count();

    occurrences >= min && occurrences <= max
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let n_valid = database
        .into_iter()
        .filter(|(min, max, l, password)| verify_policy(*min, *max, *l, password))
        .count();

    println!("{}", n_valid);

    Ok(())
}
