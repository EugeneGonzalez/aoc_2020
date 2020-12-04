use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn valid_keys(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn valid_data(&self) -> bool {
        lazy_static! {
            static ref BYR_RE: Regex = Regex::new(r"^(19[2-9]\d|200[0-2])$").unwrap();
            static ref IYR_RE: Regex = Regex::new(r"^(201\d|2020)$").unwrap();
            static ref EYR_RE: Regex = Regex::new(r"^(202\d|2030)$").unwrap();
            static ref HGT_RE: Regex =
                Regex::new(r"^((59|6\d|7[0-6])in|(1[5-8]\d|19[0-3])cm)$").unwrap();
            static ref HCL_RE: Regex = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
            static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        self.byr.as_ref().map_or(false, |val| BYR_RE.is_match(&val))
            && self.iyr.as_ref().map_or(false, |val| IYR_RE.is_match(&val))
            && self.eyr.as_ref().map_or(false, |val| EYR_RE.is_match(&val))
            && self.hgt.as_ref().map_or(false, |val| HGT_RE.is_match(&val))
            && self.hcl.as_ref().map_or(false, |val| HCL_RE.is_match(&val))
            && self.ecl.as_ref().map_or(false, |val| ECL_RE.is_match(&val))
            && self.pid.as_ref().map_or(false, |val| PID_RE.is_match(&val))
    }
}

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<Passport>;

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|entry| {
            let mut passport: Passport = Default::default();
            for (key, val) in entry
                .split_whitespace()
                .map(|pair| pair.split_at(pair.find(':').unwrap()))
            {
                let val = val[1..].trim().to_string();
                match key {
                    "byr" => passport.byr = Some(val),
                    "iyr" => passport.iyr = Some(val),
                    "eyr" => passport.eyr = Some(val),
                    "hgt" => passport.hgt = Some(val),
                    "hcl" => passport.hcl = Some(val),
                    "ecl" => passport.ecl = Some(val),
                    "pid" => passport.pid = Some(val),
                    "cid" => passport.cid = Some(val),
                    _ => panic!("Invalid key {:?} with value {:?}", key, val),
                }
            }
            passport
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|passport| passport.valid_keys())
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|passport| passport.valid_data())
        .count()
}
