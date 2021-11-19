use std::io::Read;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<Vec<String>>> {
    let mut input = BufferedInput::parse_args("Day 6: Custom Customs - Part 1")?;
    let mut whole = String::new();
    input.read_to_string(&mut whole)?;

    let groups = whole
        .split("\n\n")
        .map(|g| g.lines().map_into().collect())
        .collect();

    Ok(groups)
}

fn unique_answers(group: &[String]) -> usize {
    let mut questions = [false; 26];

    for q in group.iter().flat_map(String::as_bytes) {
        let index = (q - b'a') as usize;
        questions[index] = true;
    }

    questions.iter().filter(|&&answered| answered).count()
}

fn main() -> std::io::Result<()> {
    let groups = parse_input()?;

    aoc_utils::measure_and_print::<usize, _>(|| groups.iter().map(|g| unique_answers(g)).sum());

    Ok(())
}
