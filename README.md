# AOC 2020 Electric Rustaloo :crab:

## About

My main solution repo for Advent of Code 2020. Expect some dangerously fun Rust action!

This is mostly my attempt at polishing my Rust skills
in a controlled environment.

## Usage

I assume you know some Rust basics,
like how to get it on your computing machine (`rustup` recommended!) and how to use `cargo`.

If not, go to <https://rustup.rs/>, or check out help for `rustup` at your *nix distribution.

The solutions are all single rust programs in the [src/days/](src/days) directory.
All of them have their own binary configuration in the [Cargo.toml](Cargo.toml) file.

To run a specific solution, execute the following:

`cargo run --bin <SOLUTION_NAME> [<INPUT_FILE>]`

## aoc-utils crate

Although Rust tries to combine succinctness and efficiency,
it is in many aspects not as expressive as, say, Python.

For AOC solutions, one of the imperative things for me is to be able to read input from various sources.
That includes regular text files and STDIN, for quick dirty runs with data manually input from the keyboard.

To be able to do that without writing lots of boilerplate code in each solution,
I have devised the [aoc-utils](https://github.com/tranzystorek-io/aoc-utils) crate.
