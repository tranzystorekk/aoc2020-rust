use std::collections::HashMap;

use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<usize>> {
    let input = BufferedInput::parse_args("Day 15: Rambunctious Recitation - Part 1")?;

    let line = input.unwrapped_lines().next().unwrap();
    let sequence = line.split(',').map(|n| n.parse().unwrap()).collect();

    Ok(sequence)
}

type Entries = HashMap<usize, (usize, Option<usize>)>;

fn insert_turn(entries: &mut Entries, val: usize, turn: usize) {
    match entries.get_mut(&val) {
        Some((last, Some(before_last))) => {
            *before_last = *last;
            *last = turn;
        }
        Some((last, prev @ None)) => {
            prev.replace(*last);
            *last = turn;
        }
        _ => {
            entries.insert(val, (turn, None));
        }
    }
}

fn play_memory(nums: Vec<usize>, target_size: usize) -> usize {
    let start_size = nums.len();
    let mut last = nums.last().copied().unwrap();
    let mut visited: HashMap<usize, (usize, Option<usize>)> = nums
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, (i, None)))
        .collect();

    visited.reserve(target_size - start_size);

    for turn in start_size..target_size {
        if let Some((last_turn, Some(before_last))) = visited.get(&last).copied() {
            let age = last_turn - before_last;
            insert_turn(&mut visited, age, turn);
            last = age;
        } else {
            insert_turn(&mut visited, 0, turn);
            last = 0;
        }
    }

    last
}

fn main() -> std::io::Result<()> {
    let starting_numbers = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| play_memory(starting_numbers, 30_000_000));

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
