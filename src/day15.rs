use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::num::ParseIntError;

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<usize>;

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Result<Input, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

fn elf_game(input: &Input, target: usize) -> usize {
    let mut history: HashMap<usize, usize> = input.iter().cloned().zip(1usize..).collect();

    (input.len()..target).fold(*input.last().unwrap(), |prev, i| {
        history.insert(prev, i).map_or(0, |old_i| i - old_i)
    })
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> usize {
    elf_game(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> usize {
    elf_game(input, 30000000)
}
