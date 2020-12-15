use std::collections::HashMap;

use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<usize>> {
    let input = BufferedInput::parse_args("Day 15: Rambunctious Recitation - Part 1")?;

    let line = input.unwrapped_lines().next().unwrap();
    let sequence = line.split(',').map(|n| n.parse().unwrap()).collect();

    Ok(sequence)
}

type Entries = HashMap<usize, usize>;

fn play_memory(nums: Vec<usize>, target_size: usize) -> usize {
    let start_size = nums.len();
    let n_turns = target_size - 1;
    let mut next = nums.last().copied().unwrap();
    let mut visited: Entries = nums.into_iter().enumerate().map(|(i, n)| (n, i)).collect();

    for turn in start_size..n_turns {
        let current = next;

        next = visited
            .get(&current)
            .map(|&last_turn| turn - last_turn)
            .unwrap_or_default();

        visited.insert(current, turn);
    }

    next
}

fn main() -> std::io::Result<()> {
    let starting_numbers = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| play_memory(starting_numbers, 30_000_000));

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
