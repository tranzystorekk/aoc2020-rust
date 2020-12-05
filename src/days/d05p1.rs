use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<BoardingPass>> {
    let input = BufferedInput::parse_args("Day 5: Binary Boarding - Part 1")?;

    input
        .lines()
        .map_results(|line| BoardingPass {
            row: line[..7].into(),
            column: line[7..].into(),
        })
        .collect()
}

struct BoardingPass {
    row: String,
    column: String,
}

impl BoardingPass {
    pub fn id(&self) -> i32 {
        let r: i32 = itertools::zip((0..7).rev(), self.row.chars())
            .map(|(n, c)| {
                let bit = (c == 'B') as i32;
                bit << n
            })
            .sum();
        let c: i32 = itertools::zip((0..3).rev(), self.column.chars())
            .map(|(n, c)| {
                let bit = (c == 'R') as i32;
                bit << n
            })
            .sum();

        r * 8 + c
    }
}

fn main() -> std::io::Result<()> {
    let passes = parse_input()?;

    let max_id = passes.iter().map(BoardingPass::id).max().unwrap();

    println!("{}", max_id);

    Ok(())
}
