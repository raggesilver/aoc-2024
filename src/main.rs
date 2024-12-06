use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;

trait Day {
    fn part_one(&self, input: &str);
    fn part_two(&self, input: &str);
}

fn main() {
    // Get the day from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = match args[1].parse::<usize>() {
        Ok(num) if num >= 1 && num <= 24 => num - 1, // Convert to zero-based index
        _ => {
            eprintln!("Please provide a valid day number (1-24).");
            std::process::exit(1);
        }
    };

    // Construct the input file path
    let input_file = format!("input/day{:02}.txt", day + 1);
    let input = fs::read_to_string(&input_file).expect("Unable to read file");

    // Create a constant array of boxed trait objects
    const DAYS: [&(dyn Day + 'static); 4] =
        [&day01::Day01, &day02::Day02, &day03::Day03, &day04::Day04];

    // Call the appropriate functions based on the day
    if day < DAYS.len() {
        DAYS[day].part_one(&input);
        DAYS[day].part_two(&input);
    } else {
        eprintln!("Day {} is not implemented yet.", day + 1);
    }
}
