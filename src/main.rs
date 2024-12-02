use clap::{Parser, Subcommand};
mod days; // Import the days module

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Run Advent of Code challenges", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the solution for Day 1
    Day1,
    /// Run the solution for Day 2
    Day2,
}

fn main() {
    match Cli::parse().command {
        Commands::Day1 => days::day01::run(),
        Commands::Day2 => days::day02::run(),
    }
}
