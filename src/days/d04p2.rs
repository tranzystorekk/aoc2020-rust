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

fn validate(validated: &MaybePassport) -> bool {
    validated
        .iter()
        .filter(|&(key, val)| match key.as_str() {
            "byr" => val
                .parse::<i32>()
                .ok()
                .filter(|&n| n >= 1920 && n <= 2002)
                .is_some(),
            "iyr" => val
                .parse::<i32>()
                .ok()
                .filter(|&n| n >= 2010 && n <= 2020)
                .is_some(),
            "eyr" => val
                .parse::<i32>()
                .ok()
                .filter(|&n| n >= 2020 && n <= 2030)
                .is_some(),
            "hgt" => scan_fmt!(val, "{d}{}", i32, String)
                .ok()
                .filter(|(v, unit)| match unit.as_str() {
                    "cm" => *v >= 150 && *v <= 193,
                    "in" => *v >= 59 && *v <= 76,
                    _ => false,
                })
                .is_some(),
            "hcl" => scan_fmt!(val, "#{[0-9a-f]}", String)
                .ok()
                .filter(|color| color.len() == 6)
                .is_some(),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val.as_str()),
            "pid" => scan_fmt!(val, "{[0-9]}", String)
                .ok()
                .filter(|id| id.len() == 9)
                .is_some(),
            _ => false,
        })
        .count()
        == 7
}

fn main() -> std::io::Result<()> {
    let passes = parse_input()?;

    let n_valid = passes.into_iter().filter(validate).count();

    println!("{}", n_valid);

    Ok(())
}
