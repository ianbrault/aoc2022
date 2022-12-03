/*
** src/puzzles/mod.rs
*/

mod day_1;

use crate::types::Puzzle;

pub const N_DAYS: usize = 1;

pub const DAYS: [Puzzle; N_DAYS] = [day_1::run];
