/*
** src/puzzles/day_3.rs
** https://adventofcode.com/2022/day/3
*/

use crate::types::Solution;
use crate::utils::{self, GroupBy3};

use anyhow::Result;

use std::collections::BTreeSet;

struct Rucksack {
    compartment_a: BTreeSet<char>,
    compartment_b: BTreeSet<char>,
    full_rucksack: BTreeSet<char>,
}

impl Rucksack {
    fn common_char(&self) -> char {
        *self
            .compartment_a
            .intersection(&self.compartment_b)
            .next()
            .unwrap()
    }

    fn common_char_in_group(elf_a: &Self, elf_b: &Self, elf_c: &Self) -> char {
        let a_b_isect = elf_a
            .full_rucksack
            .intersection(&elf_b.full_rucksack)
            .cloned()
            .collect::<BTreeSet<_>>();
        *a_b_isect.intersection(&elf_c.full_rucksack).next().unwrap()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let length = s.len();
        let half = length / 2;
        let compartment_a_str = &s[..half];
        let compartment_b_str = &s[half..length];
        let compartment_a = compartment_a_str.chars().collect();
        let compartment_b = compartment_b_str.chars().collect();
        let full_rucksack = s.chars().collect();
        Self {
            compartment_a,
            compartment_b,
            full_rucksack,
        }
    }
}

fn priority(ch: char) -> u64 {
    let cn = ch as u64;
    let base_lower = 'a' as u64;
    let base_upper = 'A' as u64;
    if ch.is_lowercase() {
        cn - base_lower + 1
    } else {
        cn - base_upper + 27
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse into rucksacks
    let rucksacks = utils::split_lines(&input)
        .map(Rucksack::from)
        .collect::<Vec<_>>();

    // part 1: Find the item type that appears in both compartments of each
    // rucksack. What is the sum of the priorities of those item types?
    let priority_sum = rucksacks
        .iter()
        .map(|rucksack| rucksack.common_char())
        .map(priority)
        .sum::<u64>();
    solution.set_part_1(priority_sum);

    // part 2: Find the item type that corresponds to the badges of each
    // three-Elf group. What is the sum of the priorities of those item types?
    let elf_groups = rucksacks.iter().group_by_3().collect::<Vec<_>>();
    let group_priority_sum = elf_groups
        .iter()
        .map(|(a, b, c)| Rucksack::common_char_in_group(a, b, c))
        .map(priority)
        .sum::<u64>();
    solution.set_part_2(group_priority_sum);

    Ok(solution)
}
