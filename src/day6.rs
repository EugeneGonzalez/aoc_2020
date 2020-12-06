use aoc_runner_derive::{aoc, aoc_generator};

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<Vec<u32>>;

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.bytes().fold(0u32, |acc, x| acc | 1 << (x - b'a')))
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|group| group.iter().fold(0u32, |acc, x| acc | x).count_ones())
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|group| group.iter().fold(u32::MAX, |acc, x| acc & x).count_ones())
        .sum()
}
