/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_2;
mod day_3;
mod day_4;

use crate::types::Puzzle;

pub const N_DAYS: usize = 4;

pub const DAYS: [Puzzle; N_DAYS] = [day_1::run, day_2::run, day_3::run, day_4::run];
