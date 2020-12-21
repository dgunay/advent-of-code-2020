use std::path::Path;

mod day1;

fn load_day_input(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(
        |_| Error::InputError(format!("Couldn't load input at {}", path.to_str().unwrap()))
    )
}

#[derive(Debug)]
enum Error {
    SolutionError(i32),
    InputError(String),
}

fn main() -> Result<(), Error> {
    // TODO: parse a number from cli args
    let path_str = std::env::args()
        .nth(1)
        .ok_or(Error::InputError("No path given".to_string()))?;

    // open that day's input
    let path = Path::new(path_str.as_str());
    let input = load_day_input(path)?;

    // fuck
    let day_dir = path
    .parent()
    .unwrap()
    .iter()
    .last().unwrap().to_str().unwrap();

    let day = day_dir.parse::<i32>().expect("Failed to parse day");

    println!("Running solution for day {}", day);

    // feed it to the solution
    match day {
        1 => {
            day1::part1(input.as_str());
            todo!("part 2");
        },
        _ => {
            println!("Day {} is not implemented", day);
            std::process::exit(1);
        }
    }

    Ok(())
}
