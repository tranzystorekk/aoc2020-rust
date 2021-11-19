use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<Nav>> {
    let input = BufferedInput::parse_args("Day 12: Rain Risk - Part 1")?;

    input
        .lines()
        .map_ok(|line| {
            let (c, n) = line.split_at(1);

            match (c, n.parse().unwrap()) {
                ("N", n) => Nav::N(n),
                ("S", n) => Nav::S(n),
                ("E", n) => Nav::E(n),
                ("W", n) => Nav::W(n),
                ("L", n) => Nav::L(n),
                ("R", n) => Nav::R(n),
                ("F", n) => Nav::F(n),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Nav {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

struct NavigationChip {
    x: i32,
    y: i32,
    // 0 => N, 1 => E, 2 => S, 3 => W
    direction: i32,
}

impl NavigationChip {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: 1,
        }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn execute(&mut self, instruction: Nav) {
        match instruction {
            Nav::N(n) => self.y += n,
            Nav::S(n) => self.y -= n,
            Nav::E(n) => self.x += n,
            Nav::W(n) => self.x -= n,
            Nav::L(n) => {
                self.direction = {
                    let diff = n / 90;
                    i32::rem_euclid(self.direction - diff, 4)
                }
            }
            Nav::R(n) => {
                self.direction = {
                    let diff = n / 90;
                    i32::rem_euclid(self.direction + diff, 4)
                }
            }
            Nav::F(n) => self.move_forward(n),
        }
    }

    fn move_forward(&mut self, n: i32) {
        match self.direction {
            0 => self.y += n,
            1 => self.x += n,
            2 => self.y -= n,
            3 => self.x -= n,
            _ => unreachable!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let navigation = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut chip = NavigationChip::new();

        for instr in navigation {
            chip.execute(instr);
        }

        chip.manhattan()
    });

    Ok(())
}
