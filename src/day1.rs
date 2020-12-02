use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<HashSet<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn find_pair(nums: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    nums.iter().find_map(|num| {
        if nums.contains(&(target - num)) {
            Some((*num, (target - num)))
        } else {
            None
        }
    })
}

#[aoc(day1, part1)]
fn part1(nums: &HashSet<i32>) -> Option<i32> {
    let query = 2020;

    find_pair(nums, query).map(|(a, b)| a * b)
}

#[aoc(day1, part2)]
fn part2(nums: &HashSet<i32>) -> Option<i32> {
    let query = 2020;

    nums.iter()
        .find_map(|num| find_pair(nums, query - num).map(|(a, b)| num * a * b))
}
