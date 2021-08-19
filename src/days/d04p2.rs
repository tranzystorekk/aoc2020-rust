use std::io::Read;

use aoc_utils::BufferedInput;
use scan_fmt::scan_fmt;

type MaybePassport = Vec<(String, String)>;

fn parse_input() -> std::io::Result<Vec<MaybePassport>> {
    let mut input = BufferedInput::parse_args("Day 4: Passport Processing - Part 2")?;
    let mut whole = String::new();
    input.read_to_string(&mut whole)?;

    let passes = whole
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|kv| scan_fmt!(kv, "{}:{}", _, _).unwrap())
                .collect()
        })
        .collect();

    Ok(passes)
}

trait Validate<T>: Sized {
    fn to_option(self) -> Option<T>;

    fn validate<F: FnOnce(&T) -> bool>(self, pred: F) -> bool {
        self.to_option().filter(pred).is_some()
    }
}

impl<T, E> Validate<T> for Result<T, E> {
    fn to_option(self) -> Option<T> {
        self.ok()
    }
}

fn validate_year_range(val: &str, min: i32, max: i32) -> bool {
    val.parse::<i32>().validate(|n| (min..=max).contains(n))
}

fn validate(validated: &MaybePassport) -> bool {
    validated
        .iter()
        .filter(|&(key, val)| match key.as_str() {
            "byr" => validate_year_range(val, 1920, 2002),
            "iyr" => validate_year_range(val, 2010, 2020),
            "eyr" => validate_year_range(val, 2020, 2030),
            "hgt" => {
                scan_fmt!(val, "{d}{}", i32, String).validate(|(v, unit)| match unit.as_str() {
                    "cm" => (150..=193).contains(v),
                    "in" => (59..=76).contains(v),
                    _ => false,
                })
            }
            "hcl" => scan_fmt!(val, "#{[0-9a-f]}", String).validate(|color| color.len() == 6),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val.as_str()),
            "pid" => scan_fmt!(val, "{[0-9]}", String).validate(|id| id.len() == 9),
            _ => false,
        })
        .count()
        == 7
}

fn main() -> std::io::Result<()> {
    let passes = parse_input()?;

    let (elapsed, n_valid) = elapsed::measure_time(|| passes.into_iter().filter(validate).count());

    eprintln!("{}", elapsed);
    println!("{}", n_valid);

    Ok(())
}
