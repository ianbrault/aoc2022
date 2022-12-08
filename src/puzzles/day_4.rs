/*
** src/puzzles/day_4.rs
** https://adventofcode.com/2022/day/4
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;

type Pair = (u8, u8);

struct AssignmentPair {
    x: Pair,
    y: Pair,
}

impl AssignmentPair {
    fn parse_pair(s: &str) -> Pair {
        let split = s.find('-').unwrap();
        let a = &s[..split].parse().unwrap();
        let b = &s[(split + 1)..s.len()].parse().unwrap();
        (*a, *b)
    }

    fn pair_contains_other(&self) -> bool {
        // x is a smaller pair than y
        self.y.0 <= self.x.0 && self.y.1 >= self.x.1
    }

    fn pairs_overlap(&self) -> bool {
        if self.x.0 < self.y.0 {
            self.y.0 <= self.x.1
        } else {
            self.x.0 <= self.y.1
        }
    }
}

impl From<&str> for AssignmentPair {
    fn from(s: &str) -> Self {
        let split = s.find(',').unwrap();
        let a = Self::parse_pair(&s[..split]);
        let b = Self::parse_pair(&s[(split + 1)..s.len()]);
        // set the smaller pair as x and the larger as y
        if a.1 - a.0 < b.1 - b.0 {
            Self { x: a, y: b }
        } else {
            Self { x: b, y: a }
        }
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse into assignment pairs
    let assignment_pairs = utils::split_lines(&input)
        .map(AssignmentPair::from)
        .collect::<Vec<_>>();

    // part 1: In how many assignment pairs does one range fully contain the
    // other?
    let contain_count = assignment_pairs
        .iter()
        .filter(|x| x.pair_contains_other())
        .count();
    solution.set_part_1(contain_count);

    // part 2: In how many assignment pairs do the ranges overlap?
    let overlap_count = assignment_pairs
        .iter()
        .filter(|x| x.pairs_overlap())
        .count();
    solution.set_part_2(overlap_count);

    Ok(solution)
}
