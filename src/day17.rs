use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

// Make it easier to synchronize the input type for the generator and solvers.
type Input = HashSet<(i32, i32, i32)>;

#[aoc_generator(day17)]
fn parse_input_day17(input: &str) -> Input {
    input
        .lines()
        .zip(0i32..)
        .flat_map(|(l, y)| {
            l.bytes().zip(0i32..).filter_map(
                move |(b, x)| {
                    if b == b'#' {
                        Some((x, y, 0))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

#[rustfmt::skip]
static STEPS: [(i32, i32, i32, i32); 80] = [
    (-1, -1, -1, 0), (-1, 0, -1, 0), (-1, 1, -1, 0),
    (0, -1, -1, 0), (0, 0, -1, 0), (0, 1, -1, 0),
    (1, -1, -1, 0), (1, 0, -1, 0), (1, 1, -1, 0),
    (-1, -1, 0, 0), (-1, 0, 0, 0), (-1, 1, 0, 0),
    (0, -1, 0, 0), (0, 1, 0, 0), // Remove (0,0,0,0)
    (1, -1, 0, 0), (1, 0, 0, 0), (1, 1, 0, 0),
    (-1, -1, 1, 0), (-1, 0, 1, 0), (-1, 1, 1, 0),
    (0, -1, 1, 0), (0, 0, 1, 0), (0, 1, 1, 0),
    (1, -1, 1, 0), (1, 0, 1, 0), (1, 1, 1, 0), 
    (-1, -1, -1, -1), (-1, 0, -1, -1), (-1, 1, -1, -1),
    (0, -1, -1, -1), (0, 0, -1, -1), (0, 1, -1, -1),
    (1, -1, -1, -1), (1, 0, -1, -1), (1, 1, -1, -1),
    (-1, -1, 0, -1), (-1, 0, 0, -1), (-1, 1, 0, -1),
    (0, -1, 0, -1), (0, 0, 0, -1), (0, 1, 0, -1),
    (1, -1, 0, -1), (1, 0, 0, -1), (1, 1, 0, -1),
    (-1, -1, 1, -1), (-1, 0, 1, -1), (-1, 1, 1, -1),
    (0, -1, 1, -1), (0, 0, 1, -1), (0, 1, 1, -1),
    (1, -1, 1, -1), (1, 0, 1, -1), (1, 1, 1, -1),
    (-1, -1, -1, 1), (-1, 0, -1, 1), (-1, 1, -1, 1),
    (0, -1, -1, 1), (0, 0, -1, 1), (0, 1, -1, 1),
    (1, -1, -1, 1), (1, 0, -1, 1), (1, 1, -1, 1),
    (-1, -1, 0, 1), (-1, 0, 0, 1), (-1, 1, 0, 1),
    (0, -1, 0, 1), (0, 0, 0, 1), (0, 1, 0, 1),
    (1, -1, 0, 1), (1, 0, 0, 1), (1, 1, 0, 1),
    (-1, -1, 1, 1), (-1, 0, 1, 1), (-1, 1, 1, 1),
    (0, -1, 1, 1), (0, 0, 1, 1), (0, 1, 1, 1),
    (1, -1, 1, 1), (1, 0, 1, 1), (1, 1, 1, 1),
];

#[aoc(day17, part1)]
fn part1(input: &Input) -> usize {
    let mut neighbors = HashMap::new();
    let mut active = input.clone();

    for _ in 0..6 {
        neighbors.clear();
        for p in active.iter().flat_map(|coord| {
            STEPS[0..26]
                .iter()
                .map(move |step| (coord.0 + step.0, coord.1 + step.1, coord.2 + step.2))
        }) {
            neighbors.entry(p).and_modify(|v| *v += 1).or_insert(1);
        }

        active = neighbors
            .iter()
            .filter_map(|(k, v)| {
                if *v == 3 {
                    Some(*k)
                } else if *v == 2 && active.contains(k) {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect();
    }

    active.len()
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> usize {
    let mut neighbors = HashMap::new();
    let mut active: HashSet<(i32, i32, i32, i32)> =
        input.iter().map(|p| (p.0, p.1, p.2, 0)).collect();

    for _ in 0..6 {
        neighbors.clear();
        for p in active.iter().flat_map(|coord| {
            STEPS.iter().map(move |step| {
                (
                    coord.0 + step.0,
                    coord.1 + step.1,
                    coord.2 + step.2,
                    coord.3 + step.3,
                )
            })
        }) {
            neighbors.entry(p).and_modify(|v| *v += 1).or_insert(1);
        }

        active = neighbors
            .iter()
            .filter_map(|(k, v)| {
                if *v == 3 {
                    Some(*k)
                } else if *v == 2 && active.contains(k) {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect();
    }

    active.len()
}
