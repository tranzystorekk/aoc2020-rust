use std::io::Read;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<Vec<String>>> {
    let mut input = BufferedInput::parse_args("Day 6: Custom Customs - Part 2")?;
    let mut whole = String::new();
    input.read_to_string(&mut whole)?;

    let groups = whole
        .split("\n\n")
        .map(|g| g.lines().map_into().collect())
        .collect();

    Ok(groups)
}

fn unanimous_answers(group: &[String]) -> usize {
    let mut questions = [0; 26];
    let group_size = group.len();

    for q in group.iter().flat_map(String::as_bytes) {
        let index = (q - b'a') as usize;
        questions[index] += 1;
    }

    questions
        .iter()
        .filter(|&&n_answered| n_answered == group_size)
        .count()
}

fn main() -> std::io::Result<()> {
    let groups = parse_input()?;

    let count: usize = groups.iter().map(|g| unanimous_answers(g)).sum();

    println!("{}", count);

    Ok(())
}
