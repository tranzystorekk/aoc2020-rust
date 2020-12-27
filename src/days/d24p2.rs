use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use aoc_utils::BufferedInput;
use hex2d::{Coordinate, Direction};

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 24: Lobby Layout - Part 2")?;

    input.lines().collect()
}

type Floor = HashMap<Coordinate, bool>;

fn tile_line(description: &str) -> impl Iterator<Item = Direction> + '_ {
    let bytes = description.as_bytes();
    itertools::unfold(0, move |pos| {
        let current_pos = *pos;
        match bytes[current_pos..] {
            [b'e', ..] => {
                *pos += 1;
                Some(Direction::XY)
            }
            [b'w', ..] => {
                *pos += 1;
                Some(Direction::YX)
            }
            [b's', b'e', ..] => {
                *pos += 2;
                Some(Direction::ZY)
            }
            [b's', b'w', ..] => {
                *pos += 2;
                Some(Direction::ZX)
            }
            [b'n', b'e', ..] => {
                *pos += 2;
                Some(Direction::XZ)
            }
            [b'n', b'w', ..] => {
                *pos += 2;
                Some(Direction::YZ)
            }
            [] => None,
            _ => unreachable!(),
        }
    })
}

fn prepare_floor(instructions: Vec<String>) -> Floor {
    let mut floor = Floor::new();

    for line in instructions {
        let ref_tile = tile_line(&line).fold(Coordinate::new(0, 0), |tile, dir| tile + dir);

        let color = floor.entry(ref_tile).or_default();
        *color = !*color;
    }

    floor
}

fn next_iteration(current: &Floor) -> Floor {
    let mut white_neighbors = HashSet::new();
    let mut next = Floor::new();

    for (&pos, &state) in current {
        let mut n_black = 0;
        for &neighbor in &pos.neighbors() {
            if current.get(&neighbor).copied().unwrap_or_default() {
                n_black += 1;
            } else if state {
                white_neighbors.insert(neighbor);
            }
        }

        let new_state = match (state, n_black) {
            (true, n) if n == 0 || n > 2 => false,
            (false, 2) => true,
            (state, _) => state,
        };

        if new_state {
            next.insert(pos, new_state);
        }
    }

    for n in white_neighbors {
        let n_black = n
            .neighbors()
            .iter()
            .filter(|pos| current.get(pos).copied().unwrap_or_default())
            .count();

        let new_state = n_black == 2;

        if new_state {
            next.insert(n, new_state);
        }
    }

    next
}

fn run_exhibit(initial: Floor, n: usize) -> Floor {
    (0..n).fold(initial, |current, _| next_iteration(&current))
}

fn main() -> std::io::Result<()> {
    let lines = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let init = prepare_floor(lines);

        let after_exhibit = run_exhibit(init, 100);

        after_exhibit.values().filter(|&&color| color).count()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
