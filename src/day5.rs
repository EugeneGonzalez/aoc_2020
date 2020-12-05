use aoc_runner_derive::{aoc, aoc_generator};

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<(usize, usize)>;

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (row_str, col_str) = l.split_at(7);

            (
                row_str.chars().fold(0, |sum, c| {
                    sum * 2
                        + match c {
                            'F' => 0,
                            'B' => 1,
                            _ => panic!("Invalid char: {:?}", c),
                        }
                }),
                col_str.chars().fold(0, |sum, c| {
                    sum * 2
                        + match c {
                            'L' => 0,
                            'R' => 1,
                            _ => panic!("Invalid char: {:?}", c),
                        }
                }),
            )
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> Option<usize> {
    input.iter().map(|(row, col)| row * 8 + col).max()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    let mut vec: Vec<usize> = input.iter().map(|(row, col)| row * 8 + col).collect();
    vec.sort_unstable();
    vec.windows(2)
        .find(|window| (window[1] - window[0]) == 2)
        .unwrap()[0]
        + 1
}
