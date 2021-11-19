use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_input() -> std::io::Result<Vec<usize>> {
    let input = BufferedInput::parse_args("Day 23: Crab Cups - Part 2")?;

    let cups = input
        .unwrapped_lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    Ok(cups)
}

// the solution operates on a vector that emulates a linked list
// where each element at index v says what value comes after value v;
// e.g.the sequence {2, 1, 3, 0, 4} corresponds with the list: [4, 3, 1, 0, 2],
// that is: after 0 comes 4; after 1 comes 3; after 2 comes 1 and so on
//
// of course, the values in the puzzle start from 1 instead of 0, so they need to be adjusted
fn prepare_list(initial: Vec<usize>) -> (usize, Vec<usize>) {
    let mut result = vec![0; 1_000_000];

    let size = initial.len();
    let start = initial[0] - 1;

    let last = initial.last().copied().unwrap();

    // map out starting elements
    for (a, b) in initial.into_iter().tuple_windows() {
        result[a - 1] = b - 1;
    }

    result[last - 1] = size;

    // generate the rest of the list
    for v in (size + 1)..1_000_000 {
        result[v - 1] = v;
    }

    result[999_999] = start;

    (start, result)
}

fn get_moved(current: usize, list: &[usize]) -> [usize; 3] {
    let a = list[current];
    let b = list[a];
    let c = list[b];

    [a, b, c]
}

fn get_stars(list: &[usize]) -> (usize, usize) {
    let first = list[0];
    let second = list[first];

    (first + 1, second + 1)
}

fn next_iteration(current: usize, list: &mut Vec<usize>) -> usize {
    let moved = get_moved(current, list);

    let size = list.len();
    let mut dest = (current + size - 1) % size;

    while moved.contains(&dest) {
        dest = (dest + size - 1) % size;
    }

    // labels of interesting regions:
    // CUR -> A -> B -> C -> NXT
    // DST -> AFT
    let [a, _b, c] = moved;
    let next = list[c];
    let after_dest = list[dest];

    // after reconnections:
    // CUR -> NXT
    // DST -> A -> B -> C -> AFT

    // CUR -> NXT
    list[current] = next;

    // DST -> A
    list[dest] = a;

    // C -> AFT
    list[c] = after_dest;

    next
}

fn play_cups(mut list: Vec<usize>, current: usize, n: usize) -> Vec<usize> {
    (0..n).fold(current, |cur, _| next_iteration(cur, &mut list));

    list
}

fn main() -> std::io::Result<()> {
    let cups = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (start, list) = prepare_list(cups);

        let after = play_cups(list, start, 10_000_000);

        let (first, second) = get_stars(&after);

        first * second
    });

    Ok(())
}
