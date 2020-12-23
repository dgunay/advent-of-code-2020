/// Solution for day one
use crate::traits::Solution;
use anyhow::anyhow;
use anyhow::{Context, Result};
use std::collections::HashSet;

pub struct DayOne {}

impl Solution for DayOne {
    /// Find the two entries that sum to 2020; what do you get if you multiply them
    /// together?
    fn part1(input: &mut impl Iterator<Item = String>) -> Result<i32> {
        // Two of these entries sum to 2020
        let mut seen_numbers = std::collections::HashSet::new();

        // Read line by line
        for line in input {
            let number = line
                .parse::<i32>()
                .context(format!("Could not parse {} as an i32", line))?;

            // We can calculate the other number that will sum to 2020, and then use a
            // hash table to see if we've seen it before.
            let other = 2020 - number;

            // If we have, then return the result of multiplication.
            if seen_numbers.contains(&other) {
                return Ok(number * other);
            }

            seen_numbers.insert(number);
        }

        Err(anyhow!("Did not find two numbers summing to 2020"))
    }

    /// Okay, now what do you get if you find three numbers that sum to 2020?
    fn part2(input: &mut impl Iterator<Item = String>) -> Result<i32> {
        // We can brute force it by simply checking every number we've seen and
        // looking for the numbers summing to 2020

        // Or what about, building pairs of numbers as the inputs come in and
        // then looking for the third piece
        let mut seen = HashSet::new();
        for line in input {
            let number = line
                .parse::<i32>()
                .context(format!("Could not parse {} as an i32", line))?;

            // Just brute force compare this number to every other pair
            if seen.len() > 1 {
                for a in &seen {
                    for b in &seen {
                        // println!("Observing {} and {} with {}", a, b, number);
                        if (a + b + number) == 2020 {
                            return Ok(a * b * number);
                        }
                    }
                }
            }

            seen.insert(number);
        }

        Err(anyhow!("Did not find three numbers summing to 2020"))
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

    use super::DayOne;

    #[test]
    fn test_example() {
        let lines: Vec<String> = vec![
            "1721".to_string(),
            "979".to_string(),
            "366".to_string(),
            "299".to_string(),
            "675".to_string(),
            "1456".to_string(),
        ];
        let mut iterator = lines.into_iter();
        let mut iterator2 = iterator.clone();
        let answer = DayOne::part1(&mut iterator).unwrap();
        assert_eq!(answer, 514579);

        let p2 = DayOne::part2(&mut iterator2).unwrap();
        assert_eq!(p2, 241861950);
    }

    #[test]
    fn test_part_1() {
        let input = BufReader::new(File::open(Path::new("inputs/1/input.txt")).unwrap());
        let mut lines = input.lines().map(Result::unwrap);
        let answer = DayOne::part1(&mut lines).unwrap();
        assert_eq!(answer, 974304);
    }

    #[test]
    fn test_part_2() {
        let input = BufReader::new(File::open(Path::new("inputs/1/input.txt")).unwrap());
        let mut lines = input.lines().map(Result::unwrap);
        let answer = DayOne::part2(&mut lines).unwrap();
        assert_eq!(answer, 236430480);
    }
}
