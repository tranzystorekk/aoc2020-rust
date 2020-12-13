use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<(i32, Vec<i32>)> {
    let input = BufferedInput::parse_args("Day 13: Shuttle Search - Part 1")?;

    let mut lines = input.unwrapped_lines();

    let earliest = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    Ok((earliest, buses))
}

fn main() -> std::io::Result<()> {
    let (earliest, buses) = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let (id, departure) = buses
            .into_iter()
            .map(|id| {
                let rem = earliest % id;

                if rem == 0 {
                    (id, rem)
                } else {
                    (id, id - rem)
                }
            })
            .min_by_key(|&(_, time)| time)
            .unwrap();

        id * departure
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
