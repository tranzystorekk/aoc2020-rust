use std::{collections::HashSet, io::Read};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use lazy_static::lazy_static;
use maplit::hashset;
use scan_fmt::scan_fmt;

type MaybeFields = HashSet<String>;

fn parse_input() -> std::io::Result<Vec<MaybeFields>> {
    let mut input = BufferedInput::parse_args("Day 4: Passport Processing - Part 1")?;
    let mut whole = String::new();
    input.read_to_string(&mut whole)?;

    let passes = whole
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|kv| scan_fmt!(kv, "{}:{*}", String).unwrap())
                .collect()
        })
        .collect();

    Ok(passes)
}

fn validate(validated: &MaybeFields) -> bool {
    lazy_static! {
        static ref PASS_FIELDS: MaybeFields = hashset! {
            "byr".into(), "iyr".into(), "eyr".into(), "hgt".into(), "hcl".into(), "ecl".into(), "pid".into()
        };
    }

    PASS_FIELDS
        .difference(validated)
        .exactly_one()
        .map(|f| f == "cid")
        .unwrap_or_else(|mut rest| rest.next().is_none())
}

fn main() -> std::io::Result<()> {
    let passes = parse_input()?;

    let (elapsed, n_valid) = elapsed::measure_time(|| passes.into_iter().filter(validate).count());

    eprintln!("{}", elapsed);
    println!("{}", n_valid);

    Ok(())
}
