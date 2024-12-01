use clap::Parser;

mod days;

#[derive(Parser)]
#[command(name = "advent_of_code")]
#[command(about = "Run Advent of Code challenges", long_about = None)]
struct Cli {
    /// Specify the day to run (e.g., "day1", "day2")
    day: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.day.as_str() {
        "day1" => days::day01::run(),
        "day2" => days::day02::run(),
        _ => println!("Unknown day: {}", cli.day),
    }
}
