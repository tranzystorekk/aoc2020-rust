use std::{collections::HashSet, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<Instr>> {
    let input = BufferedInput::parse_args("Day 8: Handheld Halting - Part 2")?;

    input
        .lines()
        .map_results(|line| {
            let (instr, n) = scan_fmt!(&line, "{} {}", String, i32).unwrap();

            match instr.as_str() {
                "acc" => Instr::Acc(n),
                "jmp" => Instr::Jmp(n),
                "nop" => Instr::Nop(n),
                _ => panic!("Incorrect instruction"),
            }
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct Bootloader {
    data: Vec<Instr>,
    acc: i32,
    program_counter: usize,
    jump: Option<usize>,
}

impl Bootloader {
    pub fn new(program: Vec<Instr>) -> Self {
        Self {
            data: program,
            acc: 0,
            program_counter: 0,
            jump: None,
        }
    }

    pub fn run_till_loop_or_termination(&mut self) -> bool {
        let mut visited = HashSet::new();
        let program_size = self.data.len();

        loop {
            if self.program_counter == program_size {
                return true;
            }

            if self.program_counter > program_size {
                return false;
            }

            // check if the most recent jump address has already been visited
            if self
                .jump
                .take()
                .into_iter()
                .any(|jump_addr| !visited.insert(jump_addr))
            {
                return false;
            }

            self.step();
        }
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }

    fn step(&mut self) {
        self.exec();

        self.program_counter = self.jump.unwrap_or_else(|| self.program_counter + 1);
    }

    fn exec(&mut self) {
        match self.data[self.program_counter] {
            Instr::Acc(n) => self.acc += n,
            Instr::Jmp(n) => {
                let new_addr = self.program_counter as i32 + n;
                self.jump.replace(new_addr as usize);
            }
            Instr::Nop(_) => (),
        }
    }
}

fn main() -> std::io::Result<()> {
    use Instr::*;
    let boot_code = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        boot_code
            .iter()
            .enumerate()
            .find_map(|(i, &instr)| {
                let new_instr = match instr {
                    Jmp(n) => Nop(n),
                    Nop(n) => Jmp(n),
                    _ => return None,
                };

                let mut copy = boot_code.clone();
                copy[i] = new_instr;

                let mut bootloader = Bootloader::new(copy);
                if bootloader.run_till_loop_or_termination() {
                    Some(bootloader.acc())
                } else {
                    None
                }
            })
            .unwrap()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
