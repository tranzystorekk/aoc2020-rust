use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<(u64, u64)> {
    let input = BufferedInput::parse_args("Day 25: Combo Breaker - Part 1")?;

    let keys = input
        .unwrapped_lines()
        .map(|line| line.parse().unwrap())
        .collect_tuple()
        .unwrap();

    Ok(keys)
}

fn transforms(subject: u64) -> impl Iterator<Item = u64> {
    const MODULUS: u64 = 20201227;

    itertools::iterate(subject, move |current| current * subject % MODULUS)
}

fn main() -> std::io::Result<()> {
    let (card_pub, door_pub) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let card_loop_size = transforms(7).position(|v| v == card_pub).unwrap();

        transforms(door_pub).nth(card_loop_size).unwrap()
    });

    Ok(())
}
