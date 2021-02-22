use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Hash, Display, FromStr, PartialEq, Eq, Debug, Clone)]
#[display("{name}: {l1}-{h1} or {l2}-{h2}")]
struct Rule {
    name: String,
    l1: usize,
    h1: usize,
    l2: usize,
    h2: usize,
}

impl Rule {
    fn check(&self, num: usize) -> bool {
        (self.l1 <= num && num <= self.h1) || (self.l2 <= num && num <= self.h2)
    }
}

#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    near_tickets: Vec<Vec<usize>>,
}

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Notes;

#[aoc_generator(day16)]
fn parse_input_day16(input: &str) -> Option<Input> {
    let mut parts = input.split("\n\n");
    let rules = parts
        .next()?
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect();
    let my_ticket = parts
        .next()?
        .lines()
        .skip(1)
        .next()?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let near_tickets = parts
        .next()?
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    Some(Notes {
        rules,
        my_ticket,
        near_tickets,
    })
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    input
        .near_tickets
        .iter()
        .flatten()
        .filter(|&&x| !input.rules.iter().any(|r| r.check(x)))
        .sum()
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    // Map of value field to possible valid rules (valid rules are set to 1 in their nth bit)
    let mut rule_flags: Vec<usize> = vec![usize::MAX; input.rules.len()];

    for ticket in std::iter::once(&input.my_ticket)
        .chain(input.near_tickets.iter())
        .filter(|&ticket| {
            ticket
                .iter()
                .all(|&x| input.rules.iter().any(|r| r.check(x)))
        })
    {
        for (i, x) in ticket.iter().enumerate() {
            rule_flags[i] &= input
                .rules
                .iter()
                .enumerate()
                .fold(0usize, |flag, (j, r)| flag | ((r.check(*x) as usize) << j));
        }
    }

    // Store the value index in the vector before we reorder the vector.
    let mut rule_flags: Vec<(usize, usize)> = rule_flags.into_iter().enumerate().collect();
    rule_flags.sort_unstable_by(|a, b| a.1.count_ones().cmp(&b.1.count_ones()));

    // Since the input is doctored to be nicer, we can work from the value idx with the least
    // number of possible rules to the one with the most. Each iteration, we choose the only
    // possible rule idx that is not claimed but is possible for this value idx. Then we filter
    // if this value/rule idx pair is a depature field. If so, get the value from our ticket.
    let mut claimed = 0;
    rule_flags
        .iter()
        .filter_map(|&(value_idx, flag)| {
            let rule_idx = (!claimed & flag).trailing_zeros() as usize;
            claimed |= 1 << rule_idx;
            if input.rules[rule_idx].name.contains("departure") {
                Some(input.my_ticket[value_idx])
            } else {
                None
            }
        })
        .take(6) // Problem states only 6 depatures exist.
        .product()
}
