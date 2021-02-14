use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::error::Error;
use std::ops::Add;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{action}{units}")]
struct Movement {
    #[from_str(regex = "[a-zA-Z]")]
    action: char,
    units: isize,
}

#[derive(Debug, Default)]
struct Ship {
    x: isize,
    y: isize,
    // 0 Degrees is East with increasing values going clockwise.
    // Must be clamped to [0,360)
    direction: isize,
}

impl Ship {
    fn normalize_direction(self) -> Self {
        let (x, y) = match self.direction {
            0 => (self.x, self.y),
            90 => (self.y, -self.x),
            180 => (-self.x, -self.y),
            270 => (-self.y, self.x),
            _ => panic!("Invalid direction: {}", self.direction),
        };

        Self {
            x: x,
            y: y,
            direction: 0,
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl Add<Movement> for Ship {
    type Output = Self;

    fn add(self, other: Movement) -> Self {
        let other = if other.action == 'F' {
            Movement {
                action: match self.direction {
                    0 => 'E',
                    90 => 'S',
                    180 => 'W',
                    270 => 'N',
                    _ => panic!("Invalid direction: {}", self.direction),
                },
                units: other.units,
            }
        } else {
            other
        };

        let (x, y, direction) = match other.action {
            'N' => (self.x, self.y + other.units, self.direction),
            'S' => (self.x, self.y - other.units, self.direction),
            'E' => (self.x + other.units, self.y, self.direction),
            'W' => (self.x - other.units, self.y, self.direction),
            'L' => (
                self.x,
                self.y,
                (self.direction - other.units).rem_euclid(360),
            ),
            'R' => (
                self.x,
                self.y,
                (self.direction + other.units).rem_euclid(360),
            ),
            _ => panic!("Invalid Movement: {:?}", other),
        };

        Self {
            x: x,
            y: y,
            direction: direction,
        }
    }
}

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<Movement>;

#[aoc_generator(day12)]
fn parse_input_day12(input: &str) -> Result<Vec<Movement>, impl Error> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> isize {
    let ship: Ship = input
        .iter()
        .fold(Default::default(), |ship, movement| ship + *movement);

    ship.manhattan_distance()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> isize {
    let mut ship: Ship = Default::default();
    let mut relative = Ship {
        x: 10,
        y: 1,
        direction: 0,
    };

    for movement in input {
        if movement.action == 'F' {
            ship.x += relative.x * movement.units;
            ship.y += relative.y * movement.units;
        } else {
            relative = (relative + *movement).normalize_direction();
        }
    }

    ship.manhattan_distance()
}
