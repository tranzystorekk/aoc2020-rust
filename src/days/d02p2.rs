use aoc_utils::BufferedInput;
use itertools::Itertools;

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

            Policied {
                pos_a,
                pos_b,
                letter,
                password: pass.into(),
            }
        })
        .collect();

    Ok(result)
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
        let first = bytes[self.pos_a - 1];
        let second = bytes[self.pos_b - 1];

        [first, second]
            .iter()
            .filter(|&&b| b == self.letter)
            .count()
            == 1
    }
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let n_valid = database.into_iter().filter(Policied::verify).count();

    println!("{}", n_valid);

    Ok(())
}
