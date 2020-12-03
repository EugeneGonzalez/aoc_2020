use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid char"),
                })
                .collect()
        })
        .collect()
}

fn trees_per_slope(map: &Vec<Vec<u8>>, down: usize, right: usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;

    while row < map.len() {
        trees += map[row][col] as usize;
        col = (col + right) % map[row].len();
        row += down;
    }

    trees
}

#[aoc(day3, part1)]
fn part1(map: &Vec<Vec<u8>>) -> usize {
    trees_per_slope(map, 1, 3)
}

#[aoc(day3, part2)]
fn part2(map: &Vec<Vec<u8>>) -> usize {
    trees_per_slope(map, 1, 1)
        * trees_per_slope(map, 1, 3)
        * trees_per_slope(map, 1, 5)
        * trees_per_slope(map, 1, 7)
        * trees_per_slope(map, 2, 1)
}
