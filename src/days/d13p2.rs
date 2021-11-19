use aoc_utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<(i64, i64)>> {
    let input = BufferedInput::parse_args("Day 13: Shuttle Search - Part 2")?;

    let mut lines = input.unwrapped_lines();

    let constraints = lines
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(n, s)| {
            let id = s.parse().unwrap();
            let dep = if n != 0 {
                id - (n as i64 % id)
            } else {
                n as i64
            };

            (dep, id)
        })
        .collect();

    Ok(constraints)
}

fn mod_inverse(n: i64, modulus: i64) -> i64 {
    let mut current_mod = modulus;
    let mut current_n = n;
    let (mut x, mut y) = (1, 0);

    while current_n > 1 {
        let quotient = current_n / current_mod;

        let tmp_mod = current_mod;
        current_mod = current_n % current_mod;
        current_n = tmp_mod;

        let tmp_y = y;
        y = x - (quotient * y);
        x = tmp_y;
    }

    if x < 0 {
        return x + modulus;
    }

    x
}

fn main() -> std::io::Result<()> {
    let constraints = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let modulus: i64 = constraints.iter().map(|&(_, id)| id).product();

        // calculated with the Chinese Remainder Theory
        let departure: i64 = constraints
            .iter()
            .copied()
            .map(|(dep, id)| {
                let partial_modulus = modulus / id;
                let inverse = mod_inverse(partial_modulus, id);

                dep * partial_modulus * inverse
            })
            .sum();

        departure % modulus
    });

    Ok(())
}
