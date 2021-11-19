use std::{collections::VecDeque, io::Read};

use aoc_utils::BufferedInput;
use itertools::{izip, Itertools};

fn parse_input() -> std::io::Result<(Deck, Deck)> {
    let mut input = BufferedInput::parse_args("Day 22: Crab Combat - Part 1")?;
    let mut file = String::new();
    input.read_to_string(&mut file)?;

    let decks = file
        .split("\n\n")
        .map(|data| data.lines().skip(1).map(|n| n.parse().unwrap()).collect())
        .collect_tuple()
        .unwrap();

    Ok(decks)
}

type Deck = VecDeque<usize>;

fn play_combat(mut player1: Deck, mut player2: Deck) -> usize {
    while !player1.is_empty() && !player2.is_empty() {
        match (player1.pop_front().unwrap(), player2.pop_front().unwrap()) {
            (one, two) if one > two => {
                player1.push_back(one);
                player1.push_back(two);
            }
            (one, two) => {
                player2.push_back(two);
                player2.push_back(one);
            }
        }
    }

    let winner_ranked = if !player1.is_empty() {
        izip!((1..=player1.len()).rev(), player1)
    } else {
        izip!((1..=player2.len()).rev(), player2)
    };

    winner_ranked.map(|(rank, val)| rank * val).sum()
}

fn main() -> std::io::Result<()> {
    let (mut player1, mut player2) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        player1.reserve(player1.len());
        player2.reserve(player2.len());

        play_combat(player1, player2)
    });

    Ok(())
}
