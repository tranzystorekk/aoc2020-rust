use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<Policied>> {
    let input = BufferedInput::parse_args("Day 2: Password Philosophy - Part 1")?;

    input
        .lines()
        .map_ok(|line| {
            let (min, max, letter, password) =
                scan_fmt!(&line, "{d}-{d} {}: {}", _, _, _, _).unwrap();

            Policied {
                min,
                max,
                letter,
                password,
            }
        })
        .collect()
}

struct Policied {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Policied {
    pub fn verify(&self) -> bool {
        let occurrences = self.password.matches(self.letter).count();

        occurrences >= self.min && occurrences <= self.max
    }
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let (elapsed, n_valid) =
        elapsed::measure_time(|| database.into_iter().filter(Policied::verify).count());

    eprintln!("{}", elapsed);
    println!("{}", n_valid);

    Ok(())
}
