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
    let (mut x, mut y) = (1, 0);
    let mut current_n = n;

    while current_n > 1 {
        let q = current_n / current_mod;
        let mut t = current_mod;

        current_mod = current_n % current_mod;
        current_n = t;
        t = y;

        y = x - (q * y);
        x = t;
    }

    if x < 0 {
        x + modulus
    } else {
        x
    }
}

fn main() -> std::io::Result<()> {
    let constraints = parse_input()?;

    let (elapsed, result): (_, i64) = elapsed::measure_time(|| {
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

    eprintln!("{}", elapsed);
    println!("{}", result);

    Ok(())
}
