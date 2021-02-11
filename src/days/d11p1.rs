use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::iproduct;

type Grid = HashMap<(i32, i32), bool>;

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 11: Seating System - Part 1")?;

    input.lines().collect()
}

fn prepare_grid(rows: Vec<String>) -> Grid {
    let row_size = rows[0].len();

    rows.iter()
        .flat_map(|s| s.chars())
        .enumerate()
        .filter_map(|(i, c)| {
            let x = i % row_size;
            let y = i / row_size;

            (c == 'L').then(|| ((x as i32, y as i32), false))
        })
        .collect()
}

fn neighbors((x, y): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    let (start_x, end_x) = (x - 1, x + 1);
    let (start_y, end_y) = (y - 1, y + 1);

    iproduct!(start_x..=end_x, start_y..=end_y).filter(move |&pos| pos != (x, y))
}

fn next_iteration(current: &Grid) -> Grid {
    current
        .iter()
        .map(|(&seat_pos, &occupied)| {
            let occupied_neighbors = neighbors(seat_pos)
                .map(|pos| current.get(&pos).copied().unwrap_or_default())
                .filter(|&occ| occ)
                .count();

            let new_state = match (occupied, occupied_neighbors) {
                (false, 0) => true,
                (true, n) if n >= 4 => false,
                (state, _) => state,
            };

            (seat_pos, new_state)
        })
        .collect()
}

fn stabilize_traffic(initial: Grid) -> Grid {
    let mut current = initial;

    loop {
        let next = next_iteration(&current);

        if next == current {
            return next;
        }

        current = next;
    }
}

fn main() -> std::io::Result<()> {
    let rows = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let grid = prepare_grid(rows);

        let stabilized = stabilize_traffic(grid);

        stabilized.values().filter(|&&occupied| occupied).count()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
