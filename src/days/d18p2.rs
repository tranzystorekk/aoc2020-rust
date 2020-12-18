use std::io::BufRead;

use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<String>> {
    let input = BufferedInput::parse_args("Day 18: Operation Order - Part 2")?;

    input.lines().collect()
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Val(u64),
}

#[derive(Clone, Copy)]
enum Token {
    Op(char),
    LeftParen,
}

impl Token {
    pub fn into_op(self) -> Op {
        match self {
            Token::Op('+') => Op::Add,
            Token::Op('*') => Op::Mul,
            _ => panic!("Tried to convert inappropriate token"),
        }
    }

    pub fn is_greater_than(&self, other: &Self) -> bool {
        matches!((self, other), (Token::Op('+'), Token::Op('*')))
    }
}

fn parse_infix(s: &str) -> Vec<Op> {
    let mut result = Vec::with_capacity(s.len());
    let mut ops = Vec::with_capacity(s.len());

    let atoms = s.split_whitespace().flat_map(str::chars);
    for atom in atoms {
        match atom {
            op @ '+' | op @ '*' => {
                let opped = Token::Op(op);

                while let Some(t @ Token::Op(_)) = ops.last() {
                    if opped.is_greater_than(t) {
                        break;
                    }

                    result.push(t.into_op());
                    ops.pop();
                }
                ops.push(opped);
            }
            '(' => ops.push(Token::LeftParen),
            ')' => {
                while let Some(t @ Token::Op(_)) = ops.pop() {
                    result.push(t.into_op());
                }
            }
            n => {
                let v = n.to_digit(10).unwrap();

                result.push(Op::Val(v as u64));
            }
        }
    }

    let rest = ops.into_iter().rev().map(Token::into_op);
    result.extend(rest);

    result
}

fn eval(rpn: Vec<Op>) -> u64 {
    let mut partials = Vec::with_capacity(rpn.len());

    for el in rpn {
        match el {
            Op::Val(n) => partials.push(n),
            Op::Add => {
                let (a, b) = (partials.pop().unwrap(), partials.pop().unwrap());
                partials.push(a + b);
            }
            Op::Mul => {
                let (a, b) = (partials.pop().unwrap(), partials.pop().unwrap());
                partials.push(a * b);
            }
        }
    }

    partials.pop().unwrap()
}

fn main() -> std::io::Result<()> {
    let exprs = parse_input()?;

    let (elapsed, result): (_, u64) = elapsed::measure_time(|| {
        exprs
            .into_iter()
            .map(|expr| {
                let rpn = parse_infix(&expr);
                eval(rpn)
            })
            .sum()
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
