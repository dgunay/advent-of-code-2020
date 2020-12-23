use anyhow::Result;

// TODO: use these more if it seems to follow this format
pub trait Solution {
    // fn part1(input: &mut impl std::io::BufRead) -> Result<i32>;
    // fn part2(input: &mut impl std::io::BufRead) -> Result<i32>;
    fn part1(input: &mut impl Iterator<Item = String>) -> Result<i32>;
    fn part2(input: &mut impl Iterator<Item = String>) -> Result<i32>;
}
