use aoc_utils::BufferedInput;
use itertools::Itertools;

type Policied = (usize, usize, u8, String);

fn parse_input() -> std::io::Result<Vec<Policied>> {
    let input = BufferedInput::parse_args("Day 2: Password Philosophy - Part 2")?;

    let result = input
        .unwrapped_lines()
        .map(|line| {
            let words = line.split_whitespace();
            let (range, ch, pass) = words.collect_tuple().unwrap();

            let (pos_a, pos_b) = range
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let letter = ch.as_bytes()[0];

            (pos_a, pos_b, letter, pass.into())
        })
        .collect();

    Ok(result)
}

fn verify_policy(pos_a: usize, pos_b: usize, l: u8, password: &str) -> bool {
    let bytes = password.as_bytes();
    let first = bytes[pos_a - 1];
    let second = bytes[pos_b - 1];

    [first, second].iter().filter(|&&b| b == l).count() == 1
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let n_valid = database
        .into_iter()
        .filter(|(a, b, l, password)| verify_policy(*a, *b, *l, password))
        .count();

    println!("{}", n_valid);

    Ok(())
}
