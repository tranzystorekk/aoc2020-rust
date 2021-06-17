use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;

type Grid = HashMap<(i32, i32), bool>;

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 11: Seating System - Part 2")?;

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

fn scan_visible(grid: &Grid, (x, y): (i32, i32), w: i32, h: i32) -> usize {
    let mut n_found = 0;
    let directional_vectors = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    for (horizontal, vertical) in directional_vectors {
        let start = (x + horizontal, y + vertical);
        let found = itertools::iterate(start, |(x, y)| (x + horizontal, y + vertical))
            .take_while(|&(x, y)| x >= 0 && y >= 0 && x < w && y < h)
            .find_map(|pos| grid.get(&pos).copied())
            .filter(|&occupied| occupied)
            .is_some();

        n_found += found as usize;
    }

    n_found
}

fn next_iteration(current: &Grid, w: i32, h: i32) -> Grid {
    current
        .iter()
        .map(|(&seat_pos, &occupied)| {
            let occupied_visible = scan_visible(current, seat_pos, w, h);

            let new_state = match (occupied, occupied_visible) {
                (false, 0) => true,
                (true, n) if n >= 5 => false,
                (state, _) => state,
            };

            (seat_pos, new_state)
        })
        .collect()
}

fn stabilize_traffic(initial: Grid, w: i32, h: i32) -> Grid {
    let mut current = initial;

    loop {
        let next = next_iteration(&current, w, h);

        if next == current {
            return next;
        }

        current = next;
    }
}

fn main() -> std::io::Result<()> {
    let rows = parse_input()?;
    let (width, height) = (rows[0].len() as i32, rows.len() as i32);

    let (elapsed, result) = elapsed::measure_time(|| {
        let grid = prepare_grid(rows);

        let stabilized = stabilize_traffic(grid, width, height);

        stabilized.values().filter(|&&occupied| occupied).count()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
