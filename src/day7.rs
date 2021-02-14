use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

// Make it easier to synchronize the input type for the generator and solvers.
type Input = HashMap<String, Vec<(usize, String)>>;

fn parse_bags(bags: &str) -> Vec<(usize, String)> {
    match bags {
        "no other bags." => Vec::new(),
        _ => bags
            .split(", ")
            .map(|bag| {
                let end = bag.rfind("bag").unwrap();
                let space = bag.find(' ').unwrap();
                (
                    bag[..space].parse().unwrap(),
                    bag[space + 1..end - 1].to_owned(),
                )
            })
            .collect(),
    }
}

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let idx = line.find(" bags contain ").unwrap();
            let key = &line[..idx];
            (key.to_owned(), parse_bags(&line[idx + 14..]))
        })
        .collect()
}

fn holds_bag(key: &str, target: &str, map: &Input, cache: &mut HashMap<String, bool>) -> bool {
    // The cache already knows if this bag holds the target, then return the status.
    if let Some(&holds) = cache.get(key) {
        return holds;
    }

    // If the key is the target, update the cache and return true.
    let holds_target = if key == target {
        true
    }
    // Check if any of the internal bags hold the target.
    else if let Some(internal_bags) = map.get(key) {
        internal_bags
            .iter()
            .any(|(_count, bag)| holds_bag(bag, target, map, cache))
    } else {
        false
    };

    // If the key isn't in the cache, the target, or have any internal bags that contain the target,
    // then update the cache and return false.
    cache.insert(key.to_string(), holds_target);
    holds_target
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> usize {
    let target = "shiny gold";
    let mut cache: HashMap<String, bool> = HashMap::new();

    input
        .keys()
        .filter(|key| holds_bag(key, target, input, &mut cache))
        .count()
        - 1
}

fn count_bags(key: &str, map: &Input) -> usize {
    1 + map[key]
        .iter()
        .map(|(count, bag)| count * count_bags(bag, map))
        .sum::<usize>()
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> usize {
    count_bags("shiny gold", input) - 1
}
