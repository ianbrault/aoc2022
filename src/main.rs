/*
** src/main.rs
*/

mod puzzles;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;
use log::{debug, info};

use std::env;
use std::path::Path;

const PROJECT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

#[derive(Parser)]
struct Args {
    /// Day, runs all if not provided
    day: Option<usize>,
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

/// initializes the fern logger
fn setup_logger(debug: bool) -> Result<(), fern::InitError> {
    let level = if debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        .format(|out, message, _| {
            out.finish(format_args!(
                "[{}] {}",
                chrono::Local::now().format("%Y%m%dT%H:%M:%S"),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

/// loads puzzle input
fn load_input(day: usize) -> Result<String> {
    // get a path to the input from the top-level directory
    let input_path = Path::new(PROJECT_DIR)
        .join("input")
        .join(format!("D{}.txt", day));
    debug!(
        "loading input for day {} from {}",
        day,
        input_path.to_string_lossy()
    );
    let input = utils::read_file(&input_path)?;
    Ok(input)
}

fn run_puzzle(day: usize) -> Result<()> {
    // load the puzzle input
    let input = load_input(day)?;
    // derive the puzzle solution
    info!("Day {}", day);
    // TODO: add benchmarking code
    let solution = puzzles::DAYS[day - 1](input)?;
    if let Some(answer) = solution.part_1 {
        info!("part 1: {}", answer);
    } else {
        info!("part 1: no answer");
    }
    if let Some(answer) = solution.part_2 {
        info!("part 2: {}", answer);
    } else {
        info!("part 2: no answer");
    }
    Ok(())
}

fn main() -> Result<()> {
    // parse command-line args
    let args = Args::parse();

    // set up the logger
    if let Err(e) = setup_logger(args.debug) {
        panic!("failed to initialize logger: {}", e);
    }
    info!("Advent of Code 2022");

    if let Some(day) = args.day {
        // run a single puzzle if provided
        run_puzzle(day)?;
    } else {
        // otherwise run all puzzles
        for day in 1..=puzzles::N_DAYS {
            run_puzzle(day)?;
        }
    };

    Ok(())
}
