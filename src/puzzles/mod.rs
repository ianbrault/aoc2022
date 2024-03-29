/*
** src/puzzles/mod.rs
*/

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::types::Puzzle;

pub const N_DAYS: usize = 16;

pub const DAYS: [Puzzle; N_DAYS] = [
    day_1::run,
    day_2::run,
    day_3::run,
    day_4::run,
    day_5::run,
    day_6::run,
    day_7::run,
    day_8::run,
    day_9::run,
    day_10::run,
    day_11::run,
    day_12::run,
    day_13::run,
    day_14::run,
    day_15::run,
    day_16::run,
];
