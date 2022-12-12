/*
** src/main.rs
*/

mod puzzles;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;
use log::{debug, info, warn};

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::time::Instant;

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[cfg(feature = "sample")]
const INPUT_EXT: &str = ".dbg.txt";
#[cfg(not(feature = "sample"))]
const INPUT_EXT: &str = ".txt";

#[derive(Parser)]
struct Args {
    /// Day, runs all if not provided
    day: Option<usize>,
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
    /// Time the runtime of each puzzle
    #[arg(short, long)]
    time: bool,
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
        .join(format!("D{}{}", day, INPUT_EXT));
    debug!(
        "loading input for day {} from {}",
        day,
        input_path.to_string_lossy()
    );
    // skip if the sample input is requested but not present
    if cfg!(feature = "sample") && !input_path.exists() {
        warn!("missing sample input for day {}", day);
        Ok(String::new())
    } else {
        let input = utils::read_file(&input_path)?;
        Ok(input)
    }
}

/// runs the puzzle and returns the time elapsed as milliseconds
fn run_puzzle(day: usize) -> Result<f64> {
    // load the puzzle input
    let input = load_input(day)?;
    // skip if the sample input is requested but not present
    if cfg!(feature = "sample") && input.is_empty() {
        return Ok(0.0);
    }
    info!("Day {}", day);
    let tstart = Instant::now();
    let solution = puzzles::DAYS[day - 1](input)?;
    let duration = tstart.elapsed();
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
    Ok(duration.as_secs_f64())
}

fn main() -> Result<()> {
    // parse command-line args
    let args = Args::parse();

    // set up the logger
    if let Err(e) = setup_logger(args.debug) {
        panic!("failed to initialize logger: {}", e);
    }
    info!("Advent of Code 2022");

    // track the time elapsed for each puzzle
    let mut times = HashMap::new();

    if let Some(day) = args.day {
        // run a single puzzle if provided
        let t = run_puzzle(day)?;
        times.insert(day, t);
    } else {
        // otherwise run all puzzles
        for day in 1..=puzzles::N_DAYS {
            let t = run_puzzle(day)?;
            times.insert(day, t);
        }
    };

    // log the puzzle times, if requested
    // convert to ms for higher precision
    if args.time {
        if let Some(day) = args.day {
            info!("day {}: {:.03}ms", day, times[&day] * 1000.0);
        } else {
            // otherwise run all puzzles
            for day in 1..=puzzles::N_DAYS {
                info!("day {}: {:.03}ms", day, times[&day] * 1000.0);
            }
        };
    }

    Ok(())
}
