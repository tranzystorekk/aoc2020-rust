use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<Vec<bool>>> {
    let input = BufferedInput::parse_args("Day 3: Toboggan Trajectory - Part 1")?;

    input
        .lines()
        .map_ok(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn get_slope(w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    itertools::iterate((0, 0), move |&(x, y)| (x + w, y + h))
}

fn main() -> std::io::Result<()> {
    let land_map = parse_input()?;
    let vertical_dist = land_map.len();
    let horizontal_dist = land_map[0].len();

    let (elapsed, n_trees) = elapsed::measure_time(|| {
        get_slope(3, 1)
            .take_while(|&(_, y)| y < vertical_dist)
            .filter(|&(x, y)| {
                let row = &land_map[y];
                let mapped_x = x % horizontal_dist;

                row[mapped_x]
            })
            .count()
    });

    eprintln!("{}", elapsed);
    println!("{}", n_trees);

    Ok(())
}
