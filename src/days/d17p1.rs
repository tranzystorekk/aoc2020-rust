use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use aoc_utils::BufferedInput;
use itertools::iproduct;

type Cube = (i32, i32, i32);
type Grid = HashMap<Cube, bool>;

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 17: Conway Cubes - Part 1")?;

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

            (c == '#').then(|| ((x as i32, y as i32, 0), true))
        })
        .collect()
}

fn neighbors((x, y, z): Cube) -> impl Iterator<Item = Cube> {
    let (start_x, end_x) = (x - 1, x + 1);
    let (start_y, end_y) = (y - 1, y + 1);
    let (start_z, end_z) = (z - 1, z + 1);

    iproduct!(start_x..=end_x, start_y..=end_y, start_z..=end_z)
        .filter(move |&pos| pos != (x, y, z))
}

fn next_iteration(current: &Grid) -> Grid {
    let mut inactive_neighbors = HashSet::new();
    let mut next = Grid::new();

    for (&pos, &state) in current {
        let mut n_active = 0;
        for neighbor in neighbors(pos) {
            if current.get(&neighbor).copied().unwrap_or_default() {
                n_active += 1;
            } else if state {
                inactive_neighbors.insert(neighbor);
            }
        }

        let new_state = match (state, n_active) {
            (true, n) if !(2..=3).contains(&n) => false,
            (false, 3) => true,
            (state, _) => state,
        };

        next.insert(pos, new_state);
    }

    for n in inactive_neighbors {
        let n_active = neighbors(n)
            .filter(|pos| current.get(pos).copied().unwrap_or_default())
            .count();

        let new_state = n_active == 3;

        next.insert(n, new_state);
    }

    next
}

fn run_cycles(initial: Grid, n: usize) -> Grid {
    (0..n).fold(initial, |current, _| next_iteration(&current))
}

fn main() -> std::io::Result<()> {
    let rows = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let grid = prepare_grid(rows);

        let booted = run_cycles(grid, 6);

        booted.values().filter(|&&state| state).count()
    });

    Ok(())
}
