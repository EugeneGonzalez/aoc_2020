use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<usize>;

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Input {
    let mut input: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    input.sort_unstable();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);
    input
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> usize {
    let (diff_1, diff_3) =
        input
            .windows(2)
            .map(|window| window[1] - window[0])
            .fold((0, 0), |acc, x| {
                if x == 1 {
                    (acc.0 + 1, acc.1)
                } else if x == 3 {
                    (acc.0, acc.1 + 1)
                } else {
                    acc
                }
            });

    diff_1 * diff_3
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    cache.insert(0, 1);

    for (i, &num) in input.iter().skip(1).enumerate() {
        cache.insert(
            num,
            input[..i + 1]
                .iter()
                .rev()
                .take_while(|&x| num - x <= 3)
                .fold(0, |acc, x| acc + cache.get(x).unwrap()),
        );
    }

    *cache.get(input.last().unwrap()).unwrap()
}
