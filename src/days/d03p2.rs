use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<Vec<bool>>> {
    let input = BufferedInput::parse_args("Day 3: Toboggan Trajectory - Part 2")?;

    input
        .lines()
        .map_ok(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn get_slope(w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    itertools::iterate((0, 0), move |&(x, y)| (x + w, y + h))
}

struct LandMap {
    map: Vec<Vec<bool>>,
}

impl LandMap {
    fn count_slope_trees(&self, w: usize, h: usize) -> usize {
        let vertical_dist = self.map.len();
        let horizontal_dist = self.map[0].len();

        get_slope(w, h)
            .take_while(|&(_, y)| y < vertical_dist)
            .filter(|&(x, y)| {
                let row = &self.map[y];
                let mapped_x = x % horizontal_dist;

                row[mapped_x]
            })
            .count()
    }
}

fn main() -> std::io::Result<()> {
    let raw_map = parse_input()?;
    let land = LandMap { map: raw_map };

    let (elapsed, result): (_, usize) = elapsed::measure_time(|| {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(w, h)| land.count_slope_trees(w, h))
            .product()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
