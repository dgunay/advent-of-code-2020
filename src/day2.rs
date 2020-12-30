/// Solution for day two
use crate::traits::Solution;
use anyhow::anyhow;
use anyhow::{Context, Result};
use regex::{Captures, Regex};
use std::collections::HashSet;
use std::ops::Deref;

pub struct DayTwo {}

#[derive(Debug)]
struct Password<'a>(&'a str);

impl<'a> Password<'a> {
    pub fn new(line: &'a str) -> Result<Self> {
        // parse it out of the line
        lazy_static! {
            static ref RE: Regex = Regex::new(": (\\w+)").unwrap();
        }

        let pw = RE
            .captures(line)
            .ok_or(anyhow!("No matches"))?
            .get(1)
            .ok_or(anyhow!("Failed to match password"))?
            .as_str();

        Ok(Self { 0: &pw })
    }
}

impl<'a> Deref for Password<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Policy {
    lower_bound: u32,
    upper_bound: u32,
    character: char,
}

/// The password policy
impl Policy {
    // Parses n-n char from the string
    pub fn new(string: &str) -> Result<Self> {
        // use regex to get the lower/upper bounds and character
        lazy_static! { // compile only once
            static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\w)").unwrap();
        }

        let captures: Captures = RE
            .captures(string)
            .ok_or(anyhow!("No capture groups found"))?;

        let char_capture = captures
            .get(3)
            .ok_or(anyhow!("Unable to capture target character"))?
            .as_str();
        if char_capture.chars().count() > 1 {
            return Err(anyhow!("Too many characters in match (must be one)"));
        }

        let lower_str = captures
            .get(1)
            .ok_or(anyhow!("Unable to capture lower bound"))?
            .as_str();

        let upper_str = captures
            .get(2)
            .ok_or(anyhow!("Unable to capture upper bound"))?
            .as_str();

        Ok(Self {
            lower_bound: lower_str
                .parse::<u32>()
                .context(format!("Failed to parse {} as u32", lower_str))?,
            upper_bound: upper_str
                .parse()
                .context(format!("Failed to parse {} as u32", upper_str))?,
            character: char_capture
                .chars()
                .nth(0)
                .ok_or(anyhow!("characters is empty"))?,
        })
    }

    pub fn is_valid(&self, pw: &Password) -> bool {
        let mut occurrences = 0;

        // the char must occur between lower and upper times in the password
        for char in pw.chars() {
            if char == self.character {
                occurrences += 1;
            }

            if occurrences > self.upper_bound {
                return false;
            }
        }

        occurrences >= self.lower_bound
    }
}

impl Solution for DayTwo {
    /// How many passwords are valid, given each line's password policy?
    fn part1(input: &mut impl Iterator<Item = String>) -> Result<i32> {
        let mut valid_passwords = 0;

        for line in input {
            // format: 1-3 a: <password> is: 'a' must occur between 1-3 times in the password
            // step 1: parse the policy
            let policy = Policy::new(line.as_str())?;

            // step 2: parse the password
            let pw = Password::new(line.as_str())?;

            // step 2: check the pw
            if policy.is_valid(&pw) {
                valid_passwords += 1;
            }
        }

        Ok(valid_passwords)
    }

    /// The policy actually is positional - first digit is where to expect the
    /// character, second digit is where to _not_ expect it.
    fn part2(input: &mut impl Iterator<Item = String>) -> Result<i32> {
        let mut valid_passwords = 0;

        for line in input {
            // step 1: parse the policy
            let policy = PartTwoPolicy::new(line.as_str())?;

            // step 2: parse the password
            let pw = Password::new(line.as_str())?;

            // step 2: check the pw
            if policy.is_valid(&pw) {
                valid_passwords += 1;
            }
        }

        Ok(valid_passwords)
    }
}

struct PartTwoPolicy {
    should_be_at_index: usize,
    should_not_be_at_index: usize,
    character: char,
}

impl PartTwoPolicy {
    // Parses n-n char from the string
    pub fn new(string: &str) -> Result<Self> {
        // use regex to get the lower/upper bounds and character
        lazy_static! { // compile only once
            static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\w)").unwrap();
        }

        let captures: Captures = RE
            .captures(string)
            .ok_or(anyhow!("No capture groups found"))?;

        let char_capture = captures
            .get(3)
            .ok_or(anyhow!("Unable to capture target character"))?
            .as_str();
        if char_capture.chars().count() > 1 {
            return Err(anyhow!("Too many characters in match (must be one)"));
        }

        let should_str = captures
            .get(1)
            .ok_or(anyhow!("Unable to capture lower bound"))?
            .as_str();

        let should_not_str = captures
            .get(2)
            .ok_or(anyhow!("Unable to capture upper bound"))?
            .as_str();

        Ok(Self {
            should_be_at_index: should_str
                .parse()
                .context(format!("Failed to parse {} as usize", should_str))?,
            should_not_be_at_index: should_not_str
                .parse()
                .context(format!("Failed to parse {} as usize", should_not_str))?,
            character: char_capture
                .chars()
                .nth(0)
                .ok_or(anyhow!("characters is empty"))?,
        })
    }

    pub fn is_valid(&self, pw: &Password) -> bool {
        let mut occurrences = 0;

        let chars: Vec<char> = pw.chars().collect();

        for index in vec![self.should_be_at_index, self.should_not_be_at_index] {
            if chars.get(index - 1).copied().contains(&self.character) {
                occurrences += 1;
            }
        }

        occurrences == 1
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

        let p2 = DayTwo::part2(&mut iterator2).unwrap();
        assert_eq!(p2, 1);
    }

    #[test]
    fn test_part_1() {
        let input = BufReader::new(File::open(Path::new("inputs/2/input.txt")).unwrap());
        let mut lines = input.lines().map(Result::unwrap);
        let answer = DayTwo::part1(&mut lines).unwrap();
        assert_eq!(answer, 506);
    }

    #[test]
    fn test_part_2() {
        let input = BufReader::new(File::open(Path::new("inputs/2/input.txt")).unwrap());
        let mut lines = input.lines().map(Result::unwrap);
        let answer = DayTwo::part2(&mut lines).unwrap();
        assert_eq!(answer, 443);
    }
}
