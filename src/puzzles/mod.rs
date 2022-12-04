/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_2;

use crate::types::Puzzle;

pub const N_DAYS: usize = 2;

pub const DAYS: [Puzzle; N_DAYS] = [day_1::run, day_2::run];
