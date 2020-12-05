use std::io::BufRead;

use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 5: Binary Boarding - Part 1")?;

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

    let max_id = passes.iter().map(|pass| to_id(pass)).max().unwrap();

    println!("{}", max_id);

    Ok(())
}
