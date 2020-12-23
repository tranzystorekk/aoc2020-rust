use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<u32>> {
    let input = BufferedInput::parse_args("Day 23: Crab Cups - Part 1")?;

    let cups = input
        .unwrapped_lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    Ok(cups)
}

fn circular(seq: &[u32], start: usize) -> impl Iterator<Item = u32> + '_ {
    itertools::chain(&seq[start..], &seq[0..start]).copied()
}

fn next_iteration(seq: &[u32], current: usize) -> (Vec<u32>, usize) {
    let start = (current + 1) % seq.len();
    let current_val = seq[current];
    let mut circulate = circular(seq, start);

    let mut removed: Vec<_> = circulate.by_ref().take(3).collect();
    let mut rest: Vec<_> = circulate.collect();

    let size = seq.len() as u32;
    let mut dest = (current_val + size - 2) % size + 1;

    while removed.contains(&dest) {
        dest = (dest + size - 2) % size + 1;
    }

    let split_pos = rest
        .iter()
        .position(|&v| v == dest)
        .map(|pos| pos + 1)
        .unwrap();

    let mut result = if split_pos == rest.len() {
        rest
    } else {
        let mut split = rest.split_off(split_pos);
        split.append(&mut rest);
        split
    };

    result.append(&mut removed);

    let next_pos = result
        .iter()
        .position(|&v| v == current_val)
        .map(|pos| pos + 1)
        .unwrap();

    (result, next_pos)
}

fn play_cups(initial: Vec<u32>, n: usize) -> (Vec<u32>, usize) {
    (0..n).fold((initial, 0), |(circle, pos), _| {
        next_iteration(&circle, pos)
    })
}

fn main() -> std::io::Result<()> {
    let cups = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let (res, _) = play_cups(cups, 100);

        let pos = res.iter().position(|&v| v == 1).unwrap();
        let labels = circular(&res, pos).skip(1).join("");

        labels
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
