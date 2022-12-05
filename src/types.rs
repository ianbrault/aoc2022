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

impl From<i64> for Answer {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<usize> for Answer {
    fn from(n: usize) -> Self {
        Self::UInt(n as u64)
    }
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

impl Solution {
    pub fn new() -> Self {
        Self {
            part_1: None,
            part_2: None,
        }
    }

    pub fn set_part_1<T>(&mut self, answer: T)
    where
        T: Into<Answer>,
    {
        self.part_1 = Some(answer.into());
    }

    pub fn set_part_2<T>(&mut self, answer: T)
    where
        T: Into<Answer>,
    {
        self.part_2 = Some(answer.into());
    }
}

/// standard puzzle function type
pub type Puzzle = fn(String) -> Result<Solution>;
