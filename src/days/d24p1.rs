use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use hex2d::{Coordinate, Direction};

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 24: Lobby Layout - Part 1")?;

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

fn main() -> std::io::Result<()> {
    let lines = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let mut floor = Floor::new();

        for line in lines {
            let mut current_tile = Coordinate::new(0, 0);

            for dir in tile_line(&line) {
                current_tile = current_tile + dir;
            }

            let color = floor.entry(current_tile).or_default();
            *color = !*color;
        }

        floor.values().filter(|&&color| color).count()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
