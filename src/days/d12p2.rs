use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<Nav>> {
    let input = BufferedInput::parse_args("Day 12: Rain Risk - Part 2")?;

    input
        .lines()
        .map_results(|line| {
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

struct Position {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn execute(&mut self, instruction: Nav) {
        match instruction {
            Nav::N(n) => self.waypoint_y += n,
            Nav::S(n) => self.waypoint_y -= n,
            Nav::E(n) => self.waypoint_x += n,
            Nav::W(n) => self.waypoint_x -= n,
            Nav::L(n) => {
                let diff = n / 90;
                let rotation = i32::rem_euclid(-diff, 4);
                self.rotate_waypoint(rotation);
            }
            Nav::R(n) => {
                let diff = n / 90;
                let rotation = i32::rem_euclid(diff, 4);
                self.rotate_waypoint(rotation);
            }
            Nav::F(n) => self.move_forward(n),
        }
    }

    fn move_forward(&mut self, n: i32) {
        self.x += n * self.waypoint_x;
        self.y += n * self.waypoint_y;
    }

    // rotate clockwise by (n * 90 deg)
    fn rotate_waypoint(&mut self, n: i32) {
        match n {
            0 => (),
            1 => {
                let tmp = self.waypoint_x;
                self.waypoint_x = self.waypoint_y;
                self.waypoint_y = -tmp;
            }
            2 => {
                self.waypoint_x = -self.waypoint_x;
                self.waypoint_y = -self.waypoint_y;
            }
            3 => {
                let tmp = self.waypoint_x;
                self.waypoint_x = -self.waypoint_y;
                self.waypoint_y = tmp;
            }
            _ => unreachable!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let navigation = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        let mut position = Position::new();

        for instr in navigation {
            position.execute(instr);
        }

        position.manhattan()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
