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

fn validate_year_range(val: &str, min: i32, max: i32) -> bool {
    val.parse::<i32>()
        .ok()
        .filter(|&n| n >= min && n <= max)
        .is_some()
}

fn validate(validated: &MaybePassport) -> bool {
    validated
        .iter()
        .filter(|&(key, val)| match key.as_str() {
            "byr" => validate_year_range(val, 1920, 2002),
            "iyr" => validate_year_range(val, 2010, 2020),
            "eyr" => validate_year_range(val, 2020, 2030),
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
