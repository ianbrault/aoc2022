/*
** src/puzzles/day_1.rs
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // sum the calorie counts for each elf
    let mut elf_calories = utils::split_and_parse_lines_double::<u64>(&input)
        .iter()
        .map(|elf| elf.iter().sum::<u64>())
        .collect::<Vec<_>>();
    elf_calories.sort();
    let n_elves = elf_calories.len();

    // part 1: Find the Elf carrying the most Calories. How many total Calories
    // is that Elf carrying?
    let elf_most_cals = elf_calories[n_elves - 1];
    solution.set_part_1(elf_most_cals);

    // part 2: Find the top three Elves carrying the most Calories. How many
    // Calories are those Elves carrying in total?
    let elf_top_3_cals = elf_calories[(n_elves - 3)..n_elves].iter().sum::<u64>();
    solution.set_part_2(elf_top_3_cals);

    Ok(solution)
}
