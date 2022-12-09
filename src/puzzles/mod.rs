/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use crate::types::Puzzle;

pub const N_DAYS: usize = 8;

pub const DAYS: [Puzzle; N_DAYS] = [
    day_1::run,
    day_2::run,
    day_3::run,
    day_4::run,
    day_5::run,
    day_6::run,
    day_7::run,
    day_8::run,
];
