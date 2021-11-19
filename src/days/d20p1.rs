use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse_input() -> std::io::Result<Vec<(u64, Image)>> {
    let mut input = BufferedInput::parse_args("Day 20: Jurassic Jigsaw - Part 1")?;
    let mut file = String::new();
    input.read_to_string(&mut file)?;

    let split = file.split("\n\n");

    let result = split
        .map(|data| {
            let mut lines = data.lines();

            let id_line = lines.next().unwrap();
            let id = scan_fmt!(id_line, "Tile {d}:", _).unwrap();

            let img = lines.map_into().collect();

            (id, img)
        })
        .collect();

    Ok(result)
}

fn get_borders(image: &[Vec<u8>]) -> HashSet<Vec<u8>> {
    let width = image[0].len();
    let mut result = HashSet::with_capacity(8);

    let indices = [0, width - 1];
    let sides = indices
        .iter()
        .map(|&idx| image.iter().map(|line| line[idx]).collect());

    let x = [image.first().unwrap(), image.last().unwrap()];
    let borders = x.iter().map(|&s| s.clone());

    result.extend(sides);
    result.extend(borders);

    let reversed: Vec<_> = result
        .iter()
        .map(|s| s.iter().copied().rev().collect())
        .collect();

    result.extend(reversed);

    result
}

type Image = Vec<Vec<u8>>;

fn main() -> std::io::Result<()> {
    let images = parse_input()?;

    aoc_utils::measure_and_print::<u64, _>(|| {
        let bordered: Vec<_> = images
            .iter()
            .map(|(id, img)| {
                let borders = get_borders(img);

                (*id, borders)
            })
            .collect();

        let mut checker: HashMap<u64, usize> = HashMap::new();

        for (i, j) in (0..bordered.len()).tuple_combinations() {
            let (id_a, set_a) = &bordered[i];
            let (id_b, set_b) = &bordered[j];

            if !set_a.is_disjoint(set_b) {
                *checker.entry(*id_a).or_default() += 1;
                *checker.entry(*id_b).or_default() += 1;
            }
        }

        checker
            .iter()
            .filter_map(|(&id, &n)| (n == 2).then(|| id))
            .product()
    });

    Ok(())
}
