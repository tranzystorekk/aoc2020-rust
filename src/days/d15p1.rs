use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 15: Rambunctious Recitation - Part 1")?;

    let line = input.unwrapped_lines().next().unwrap();
    let sequence = line.split(',').map(|n| n.parse().unwrap()).collect();

    Ok(sequence)
}

fn play_memory(mut nums: Vec<i32>, target_size: usize) -> Vec<i32> {
    nums.reserve(target_size);

    while nums.len() < target_size {
        let recent = nums.last().unwrap();

        let next = nums
            .iter()
            .rev()
            .skip(1)
            .position(|n| n == recent)
            .map(|n| n + 1)
            .unwrap_or_default();

        nums.push(next as i32);
    }

    nums
}

fn main() -> std::io::Result<()> {
    let starting_numbers = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let all_nums = play_memory(starting_numbers, 2020);

        all_nums.last().copied().unwrap()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
