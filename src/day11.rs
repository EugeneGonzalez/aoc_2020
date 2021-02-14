use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Floor,
    Empty,
    Occupied,
}

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<Vec<Space>>;

#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Space::Floor,
                    'L' => Space::Empty,
                    '#' => Space::Occupied,
                    _ => panic!("Invalid char"),
                })
                .collect()
        })
        .collect()
}

static DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn run_sim(floor: &mut Input, floor_buf: &mut Input) -> usize {
    let mut count = 0;

    for (r, row) in floor.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let mut neighbors = DIRECTIONS.iter().filter_map(|&(dx, dy)| {
                floor
                    .get((r as isize + dy) as usize)?
                    .get((c as isize + dx) as usize)
            });
            floor_buf[r][c] = match val {
                Space::Empty => {
                    if neighbors.all(|&val| val != Space::Occupied) {
                        count += 1;
                        Space::Occupied
                    } else {
                        Space::Empty
                    }
                }
                Space::Occupied => {
                    if neighbors.filter(|&&val| val == Space::Occupied).count() < 4 {
                        count += 1;
                        Space::Occupied
                    } else {
                        Space::Empty
                    }
                }
                &x => x,
            };
        }
    }

    std::mem::swap(floor, floor_buf);
    count
}

fn run_sim2(floor: &mut Input, floor_buf: &mut Input) -> usize {
    let mut count = 0;

    for (r, row) in floor.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let mut neighbors = DIRECTIONS.iter().filter_map(|&(dx, dy)| {
                let mut y: isize = r as isize;
                let mut x: isize = c as isize;
                loop {
                    y += dy;
                    x += dx;
                    let val = *floor.get(y as usize)?.get(x as usize)?;

                    if val != Space::Floor {
                        return Some(val);
                    }
                }
            });
            floor_buf[r][c] = match val {
                Space::Empty => {
                    if neighbors.all(|val| val != Space::Occupied) {
                        count += 1;
                        Space::Occupied
                    } else {
                        Space::Empty
                    }
                }
                Space::Occupied => {
                    if neighbors.filter(|&val| val == Space::Occupied).count() < 5 {
                        count += 1;
                        Space::Occupied
                    } else {
                        Space::Empty
                    }
                }
                &x => x,
            };
        }
    }

    std::mem::swap(floor, floor_buf);
    count
}

#[aoc(day11, part1)]
fn part1(input: &Input) -> usize {
    let mut count = 1;
    let mut old_count = 0;
    let floor: &mut Input = &mut input.clone();
    let floor_buf: &mut Input = &mut input.clone();

    while count != old_count {
        old_count = count;
        count = run_sim(floor, floor_buf);
    }

    count
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> usize {
    let mut count = 1;
    let mut old_count = 0;
    let floor: &mut Input = &mut input.clone();
    let floor_buf: &mut Input = &mut input.clone();

    while count != old_count {
        old_count = count;
        count = run_sim2(floor, floor_buf);
    }

    count
}
