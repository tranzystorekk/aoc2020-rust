use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 5: Binary Boarding - Part 2")?;

    input.lines().collect()
}

fn to_id(pass: &str) -> i32 {
    itertools::zip((0..10).rev(), pass.chars())
        .map(|(n, c)| {
            let bit = ['B', 'R'].contains(&c) as i32;
            bit << n
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let passes = parse_input()?;

    let (elapsed, missing_id) = elapsed::measure_time(|| {
        passes
            .iter()
            .map(|pass| to_id(pass))
            .sorted_unstable()
            .tuple_windows()
            .find_map(|(a, b)| Some(a + 1).filter(|_| b != a + 1))
            .unwrap()
    });

    eprintln!("{}", elapsed);
    println!("{}", missing_id);

    Ok(())
}
