use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<Policied>> {
    let input = BufferedInput::parse_args("Day 2: Password Philosophy - Part 2")?;

    input
        .lines()
        .map_results(|line| {
            let (pos_a, pos_b, l, password) =
                scan_fmt!(&line, "{d}-{d} {}: {}", _, _, char, _).unwrap();

            Policied {
                pos_a,
                pos_b,
                letter: l as u8,
                password,
            }
        })
        .collect()
}

struct Policied {
    pos_a: usize,
    pos_b: usize,
    letter: u8,
    password: String,
}

impl Policied {
    pub fn verify(&self) -> bool {
        let bytes = self.password.as_bytes();
        let check_a = bytes[self.pos_a - 1] == self.letter;
        let check_b = bytes[self.pos_b - 1] == self.letter;

        check_a ^ check_b
    }
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let n_valid = database.into_iter().filter(Policied::verify).count();

    println!("{}", n_valid);

    Ok(())
}
