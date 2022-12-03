/*
** src/types.rs
*/

use anyhow::Result;

use std::fmt;

/// sum type for all possible puzzle answers
pub enum Answer {
    Int(i64),
    UInt(u64),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", x),
            Self::UInt(x) => write!(f, "{}", x),
        }
    }
}

/// holds parts 1 and 2 answers to a puzzle
pub struct Solution {
    pub part_1: Option<Answer>,
    pub part_2: Option<Answer>,
}

/// standard puzzle function type
pub type Puzzle = fn(String) -> Result<Solution>;
