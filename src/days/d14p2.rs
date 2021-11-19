use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<Instr>> {
    let input = BufferedInput::parse_args("Day 14: Docking Data - Part 2")?;

    input
        .lines()
        .map_ok(|line| {
            if line.starts_with("mask") {
                let mask_string = scan_fmt!(&line, "mask = {}", String).unwrap();
                let mut mask = [MaskBit::None; 36];

                let bits = mask_string.bytes().map(|b| match b {
                    b'0' => MaskBit::Zero,
                    b'1' => MaskBit::One,
                    _ => MaskBit::None,
                });
                mask.iter_mut().set_from(bits);

                Instr::Mask(mask)
            } else {
                let (addr, val) = scan_fmt!(&line, "mem[{d}] = {d}", _, _).unwrap();
                Instr::MemSet(addr, val)
            }
        })
        .collect()
}

type Mask = [MaskBit; 36];
type Memory = HashMap<u64, u64>;

#[derive(Copy, Clone)]
enum MaskBit {
    None,
    Zero,
    One,
}

#[derive(Copy, Clone)]
enum Instr {
    Mask(Mask),
    MemSet(u64, u64),
}

struct DockingComputer {
    program: Vec<Instr>,
    memory: Memory,
    mask: Mask,
}

impl DockingComputer {
    pub fn new(program: Vec<Instr>) -> Self {
        Self {
            program,
            memory: HashMap::new(),
            mask: [MaskBit::None; 36],
        }
    }

    pub fn execute(&mut self) {
        let size = self.program.len();
        for i in 0..size {
            match self.program[i] {
                Instr::Mask(mask) => self.mask = mask,
                Instr::MemSet(addr, val) => self.set(addr, val),
            }
        }
    }

    pub fn memory_summed(&self) -> u64 {
        self.memory.values().sum()
    }

    fn set(&mut self, addr: u64, value: u64) {
        for addr in self.mask_address(addr) {
            self.memory.insert(addr, value);
        }
    }

    fn mask_address(&self, mut address: u64) -> impl Iterator<Item = u64> {
        let floating_bits: Vec<_> = self
            .mask
            .iter()
            .copied()
            .rev()
            .enumerate()
            .filter_map(|v| {
                match v {
                    (n, MaskBit::None) => {
                        return Some(n);
                    }
                    (n, MaskBit::One) => {
                        let mask = 1 << n;
                        address |= mask;
                    }
                    _ => (),
                }

                None
            })
            .collect();

        // generate an iterator of all possible addresses
        let max: u64 = 1 << floating_bits.len();
        (0..max).scan((floating_bits, address), |(positions, addr), floats| {
            let mut new_addr = *addr;

            for (i, pos) in positions.iter().copied().enumerate() {
                let bit = (floats >> i) & 1;

                match bit {
                    0 => {
                        let mask = !(1 << pos);
                        new_addr &= mask;
                    }
                    1 => {
                        let mask = 1 << pos;
                        new_addr |= mask;
                    }
                    _ => unreachable!(),
                }
            }

            Some(new_addr)
        })
    }
}

fn main() -> std::io::Result<()> {
    let init_program = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut computer = DockingComputer::new(init_program);

        computer.execute();

        computer.memory_summed()
    });

    Ok(())
}
