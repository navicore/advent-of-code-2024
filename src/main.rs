use clap::{Parser, Subcommand};
mod days; // Import the days module

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Run Advent of Code challenges")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1,
    Day2,
    Day3,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Commands::Day1 => days::day01::run(),
        Commands::Day2 => days::day02::run(),
        Commands::Day3 => days::day03::run()?,
    }
    Ok(())
}
