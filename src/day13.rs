use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct InputDay13 {
    arrival: i64,
    buses: Vec<(i64, i64)>,
}

// Make it easier to synchronize the input type for the generator and solvers.
type Input = InputDay13;

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> Input {
    let mut lines = input.lines();

    InputDay13 {
        arrival: lines.next().unwrap().parse().unwrap(),
        buses: lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(x, s)| s.parse().ok().map(|id| (x as i64, id)))
            .collect(),
    }
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> Option<i64> {
    (input.arrival..).find_map(|time| {
        input
            .buses
            .iter()
            .find(|(_x, id)| time % id == 0)
            .map(|(_x, id)| id * (time - input.arrival))
    })
}

// Code for egcd and mod_inv from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> i64 {
    let big_prod = input.buses.iter().map(|&(_x, id)| id).product::<i64>();

    input
        .buses
        .iter()
        .map(|&(delay, n_i)| {
            let y_i = big_prod / n_i;

            // -delay -> (n_i - delay) % n_i -> a_i in chinese rmainder theorem
            // Also the inputs are dockered so g == 1 in mod_inv
            -delay * y_i * mod_inv(y_i, n_i).unwrap()
        })
        .sum::<i64>()
        .rem_euclid(big_prod)
}
