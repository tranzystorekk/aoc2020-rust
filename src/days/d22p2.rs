use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

use aoc_utils::BufferedInput;
use itertools::{izip, Itertools};

fn parse_input() -> std::io::Result<(Deck, Deck)> {
    let mut input = BufferedInput::parse_args("Day 22: Crab Combat - Part 2")?;
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

#[derive(Clone, Copy)]
enum Winner {
    One,
    Two,
}

fn play_combat(player1: Deck, player2: Deck) -> usize {
    let mut games = vec![((player1, player2), None, HashSet::new())];
    let mut last_winner = None;

    loop {
        let (players, prev_draw, prev_rounds) = games.last_mut().unwrap();

        match prev_draw.take() {
            // play another standard round
            None => {
                if prev_rounds.contains(players) {
                    last_winner.replace(Winner::One);
                    games.pop();
                    continue;
                }

                prev_rounds.insert(players.clone());

                let (player1, player2) = players;

                let current_draw = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
                match current_draw {
                    (one, two) if player1.len() >= one && player2.len() >= two => {
                        let p1_copy = player1.iter().copied().take(one).collect();
                        let p2_copy = player2.iter().copied().take(two).collect();

                        prev_draw.replace(current_draw);

                        games.push(((p1_copy, p2_copy), None, HashSet::new()));
                        continue;
                    }
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
            // resolve round via sub-game result
            Some((one, two)) => {
                let (player1, player2) = players;

                match last_winner.take().unwrap() {
                    Winner::One => {
                        player1.push_back(one);
                        player1.push_back(two);
                    }
                    Winner::Two => {
                        player2.push_back(two);
                        player2.push_back(one);
                    }
                }
            }
        }

        let player1_empty = players.0.is_empty();
        let player2_empty = players.1.is_empty();

        if !player1_empty && !player2_empty {
            continue;
        }

        let winner = if !player1_empty {
            &players.0
        } else {
            &players.1
        };

        let score = izip!((1..=winner.len()).rev(), winner)
            .map(|(pos, val)| pos * val)
            .sum();

        // finally, a global-game victory
        if games.len() == 1 {
            return score;
        }

        // mark this sub-game's winner
        if !player1_empty {
            last_winner.replace(Winner::One);
        } else {
            last_winner.replace(Winner::Two);
        }

        games.pop();
    }
}

fn main() -> std::io::Result<()> {
    let (mut player1, mut player2) = parse_input()?;

    let (elapsed, result) = elapsed::measure_time(|| {
        player1.reserve(player1.len());
        player2.reserve(player2.len());

        play_combat(player1, player2)
    });

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
