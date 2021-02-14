use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use std::num::ParseIntError;

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<usize>;

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Result<Input, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> Option<usize> {
    input.windows(26).find_map(|window| {
        let num = *window.last()?;

        if !window[..25]
            .iter()
            .tuple_combinations()
            .any(|(&a, &b)| a + b == num)
        {
            Some(num)
        } else {
            None
        }
    })
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> usize {
    let invalid_num = 217430975;
    for idx in 0..input.len() {
        let mut sum = input[idx];
        let mut len = 1;

        while sum < invalid_num {
            sum += input[idx + len];
            if invalid_num == sum {
                match input[idx..idx + len + 1].iter().minmax() {
                    MinMax(x, y) => return x + y,
                    _ => panic!("Didn't find a pair."),
                }
            }

            len += 1;
        }
    }

    panic!("Didn't find a contiguous sum to invalid number.");
}
