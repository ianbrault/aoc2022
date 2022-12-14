/*
** src/types.rs
*/

use anyhow::Result;

use std::error;
use std::fmt;

/// sum type for all possible puzzle answers
pub enum Answer {
    Int(i64),
    UInt(u64),
    Str(String),
}

impl From<i64> for Answer {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<i32> for Answer {
    fn from(n: i32) -> Self {
        Self::Int(n as i64)
    }
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<u32> for Answer {
    fn from(n: u32) -> Self {
        Self::UInt(n as u64)
    }
}

impl From<usize> for Answer {
    fn from(n: usize) -> Self {
        Self::UInt(n as u64)
    }
}

impl From<String> for Answer {
    fn from(n: String) -> Self {
        Self::Str(n)
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", x),
            Self::UInt(x) => write!(f, "{}", x),
            Self::Str(x) => write!(f, "{}", x),
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

/// custom error type
#[derive(Debug)]
pub enum Error {
    NoSolution,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoSolution => write!(f, "no solution found"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Self::NoSolution => "no solution found",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn manhattan_distance(point_a: Self, point_b: Self) -> i64 {
        let dx = point_a.x - point_b.x;
        let dy = point_a.y - point_b.y;
        dx.abs() + dy.abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
