/// Solution for day two
use crate::traits::Solution;
use anyhow::anyhow;
use anyhow::{Context, Result};
use std::collections::HashSet;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

pub struct DayTwo {}

struct Policy {
    lower_bound: u32,
    upper_bound: u32,
    character: char
}


impl Policy {
    // Parses n-n char from the string
    pub fn new(str: &str) -> Self {
        // use regex to get the lower/upper bounds and character
        lazy_static! { // compile only once
            static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\w)").unwrap();
        }

        let captures = RE.captures()
        Self {

        }
    }
}

impl Solution for DayTwo {
    /// How many passwords are valid, given each line's password policy?
    fn part1(input: &mut impl Iterator<Item = String>) -> Result<i32> {
        for line in input {
            // format: 1-3 a: <password> is: 'a' must occur between 1-3 times in the password

            // step 1: parse the policy

            // step 2: check the pw
        }
    }

    fn part2(input: &mut impl Iterator<Item = String>) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
        vec,
    };

    use crate::traits::Solution;

    use super::DayTwo;

    #[test]
    fn test_example() {
        let lines: Vec<String> = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let mut iterator = lines.into_iter();
        let mut iterator2 = iterator.clone();
        let answer = DayTwo::part1(&mut iterator).unwrap();
        assert_eq!(answer, 2);

        // let p2 = DayTwo::part2(&mut iterator2).unwrap();
        // assert_eq!(p2, 241861950);
    }

    #[test]
    fn test_part_1() {
        let input = BufReader::new(File::open(Path::new("inputs/2/input.txt")).unwrap());
        let mut lines = input.lines().map(Result::unwrap);
        let answer = DayTwo::part1(&mut lines).unwrap();
        // assert_eq!(answer, 974304);
    }

    #[test]
    fn test_part_2() {
        let input = BufReader::new(File::open(Path::new("inputs/2/input.txt")).unwrap());
        let mut lines = input.lines().map(Result::unwrap);
        let answer = DayTwo::part2(&mut lines).unwrap();
        // assert_eq!(answer, 236430480);
    }
}
