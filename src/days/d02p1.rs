use aoc_utils::BufferedInput;
use itertools::Itertools;

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

            Policied {
                min,
                max,
                letter,
                password: pass.into(),
            }
        })
        .collect();

    Ok(result)
}

struct Policied {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Policied {
    pub fn verify(&self) -> bool {
        let occurrences = self.password.chars().filter(|&c| c == self.letter).count();

        occurrences >= self.min && occurrences <= self.max
    }
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let n_valid = database.into_iter().filter(Policied::verify).count();

    println!("{}", n_valid);

    Ok(())
}
