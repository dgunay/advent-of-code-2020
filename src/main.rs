#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;
mod traits;
mod util;

use crate::traits::Solution;
use anyhow::anyhow;
use anyhow::Result;
use std::{fs::File, io::prelude::*, io::BufReader, path::Path};

fn main() -> Result<()> {
    // TODO: parse a number from cli args
    let path_str = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No path given".to_string()))?;

    // open that day's input
    let path = Path::new(path_str.as_str());
    let input = BufReader::new(File::open(path)?);

    // fuck
    let day_dir = path
        .parent()
        .unwrap()
        .iter()
        .last()
        .unwrap()
        .to_str()
        .unwrap();

    let day = day_dir.parse::<i32>().expect("Failed to parse day");

    println!("Running solution for day {}", day);

    let lines: Vec<String> = input.lines().map(Result::unwrap).collect();
    let mut iterator = lines.into_iter();
    let mut iterator2 = iterator.clone();
    // feed it to the solution
    match day {
        1 => {
            let solution = day1::DayOne::part1(&mut iterator)?;
            println!("Part 1: {}", solution);
            let solution2 = day1::DayOne::part2(&mut iterator2)?;
            println!("Part 2: {}", solution2);
            // todo!()
        }
        _ => {
            println!("Day {} is not implemented", day);
            std::process::exit(1);
        }
    };

    Ok(())
}
