use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::error::Error;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<PasswordRule>, impl Error> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day2, part1)]
fn part1(rules: &Vec<PasswordRule>) -> usize {
    rules
        .iter()
        .filter(|rule| (rule.min..=rule.max).contains(&rule.password.matches(rule.letter).count()))
        .count()
}

#[aoc(day2, part2)]
fn part2(rules: &Vec<PasswordRule>) -> usize {
    rules
        .iter()
        .filter(|rule| {
            let first = if let Some(c) = rule.password.chars().nth(rule.min - 1) {
                c == rule.letter
            } else {
                false
            };

            let second = if let Some(c) = rule.password.chars().nth(rule.max - 1) {
                c == rule.letter
            } else {
                false
            };

            first ^ second
        })
        .count()
}
