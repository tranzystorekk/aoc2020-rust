use aoc_utils::BufferedInput;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<Policied>> {
    let input = BufferedInput::parse_args("Day 2: Password Philosophy - Part 1")?;

    let result = input
        .unwrapped_lines()
        .map(|line| {
            let (min, max, letter, password) =
                scan_fmt!(&line, "{d}-{d} {}: {}", usize, usize, char, String).unwrap();

            Policied {
                min,
                max,
                letter,
                password,
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
        let occurrences = self.password.matches(self.letter).count();

        occurrences >= self.min && occurrences <= self.max
    }
}

fn main() -> std::io::Result<()> {
    let database = parse_input()?;

    let n_valid = database.into_iter().filter(Policied::verify).count();

    println!("{}", n_valid);

    Ok(())
}
