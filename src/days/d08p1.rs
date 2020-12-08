use std::{collections::HashSet, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<Instr>> {
    let input = BufferedInput::parse_args("Day 8: Handheld Halting - Part 1")?;

    input
        .lines()
        .map_results(|line| {
            let (instr, n) = scan_fmt!(&line, "{} {}", String, i32).unwrap();

            match instr.as_str() {
                "acc" => Instr::Acc(n),
                "jmp" => Instr::Jmp(n),
                "nop" => Instr::Nop,
                _ => panic!("Incorrect instruction"),
            }
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop,
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

    pub fn run_till_loop(&mut self) {
        let mut visited = HashSet::new();

        loop {
            if !visited.insert(self.program_counter) {
                return;
            }

            self.step();
        }
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }

    fn step(&mut self) {
        self.exec();

        self.program_counter = self.jump.take().unwrap_or(self.program_counter + 1);
    }

    fn exec(&mut self) {
        match self.data[self.program_counter] {
            Instr::Acc(n) => self.acc += n,
            Instr::Jmp(n) => {
                let new_addr = self.program_counter as i32 + n;
                self.jump.replace(new_addr as usize);
            }
            Instr::Nop => (),
        }
    }
}

fn main() -> std::io::Result<()> {
    let boot_code = parse_input()?;

    let mut bootloader = Bootloader::new(boot_code);
    bootloader.run_till_loop();

    let result = bootloader.acc();

    println!("{}", result);

    Ok(())
}
