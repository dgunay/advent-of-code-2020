// use anyhow::{Context, Result};
// use std::path::Path;

// pub fn load_day_input(path: &Path) -> Result<String> {
//     std::fs::read_to_string(path).context("failed to read path into string")
// }

// pub fn get_input_iterator(reader: &impl BufRead) -> impl Iterator {
//     reader.lines().map(|result| result.context("Could not read line").unwrap())
// }
