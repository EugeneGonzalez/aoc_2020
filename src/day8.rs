use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;

#[derive(Debug)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<Op>;

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let val = line[4..].parse().unwrap();
            match &line[..3] {
                "nop" => Op::Nop(val),
                "acc" => Op::Acc(val),
                "jmp" => Op::Jmp(val),
                _ => panic!("Invalid instruction: {:?}", line),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> isize {
    let mut acc = 0;
    let mut pc = 0;
    let mut visted = BitSet::with_capacity(input.len());

    while visted.insert(pc) {
        match input[pc] {
            Op::Acc(val) => {
                acc += val;
                pc += 1;
            }
            Op::Nop(_) => {
                pc += 1;
            }
            Op::Jmp(val) => pc = (pc as isize + val) as usize,
        }
    }

    acc
}

fn run_prog_with_swap(input: &Input, swap_pc: usize) -> Option<isize> {
    let mut acc = 0;
    let mut pc = 0;
    let mut visted = BitSet::with_capacity(input.len());

    while visted.insert(pc) {
        match (&input[pc], pc == swap_pc) {
            (Op::Acc(val), _) => acc += val,
            (Op::Nop(val), true) | (Op::Jmp(val), false) => pc = (pc as isize + val - 1) as usize,
            _ => (),
        }
        pc += 1;

        if pc == input.len() {
            return Some(acc);
        }
    }

    None
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> isize {
    (0..input.len())
        .filter(|&pc| match input[pc] {
            Op::Acc(_) => false,
            Op::Nop(_) | Op::Jmp(_) => true,
        })
        .filter_map(|pc| run_prog_with_swap(input, pc))
        .next()
        .unwrap()
}
