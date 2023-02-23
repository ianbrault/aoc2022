/*
** src/utils.rs
*/

use anyhow::Result;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::{FromStr, Split};

/// reads the contents of a file into a string
pub fn read_file(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// splits a string by newlines
pub fn split_lines(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n')
}

/// splits a string by chunks of newlines, separated by double newlines
pub fn split_lines_double(input: &str) -> impl Iterator<Item = Split<'_, char>> {
    input.split("\n\n").map(|chunk| chunk.split('\n'))
}

/// splits a string by chunks of newlines, separated by double newlines, and
/// parses a type out of the strings
pub fn split_and_parse_lines_double<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    split_lines_double(input)
        .map(|chunk| chunk.map(|s| s.parse::<T>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

/// grabs the n-th character from the given string
pub fn nchar(s: &str, n: usize) -> char {
    s.chars().nth(n).unwrap()
}

/// finds the first index of the character in the given string
pub fn find_char(s: &str, c: char) -> Option<usize> {
    s.chars().position(|cc| cc == c)
}

/// iterator adapter to group an iterator into 2-tuples
pub struct GroupBy2Iterator<I> {
    iter: I,
}

impl<I> GroupBy2Iterator<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I, T> Iterator for GroupBy2Iterator<I>
where
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        let y = self.iter.next();
        if let (Some(a), Some(b)) = (x, y) {
            Some((a, b))
        } else {
            None
        }
    }
}

pub trait GroupBy2<T>: Iterator<Item = T> + Sized {
    fn group_by_2(self) -> GroupBy2Iterator<Self> {
        GroupBy2Iterator::new(self)
    }
}

impl<T, I: Iterator<Item = T>> GroupBy2<T> for I {}

/// iterator adapter to group an iterator into 3-tuples
pub struct GroupBy3Iterator<I> {
    iter: I,
}

impl<I> GroupBy3Iterator<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I, T> Iterator for GroupBy3Iterator<I>
where
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    type Item = (&'a T, &'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        let y = self.iter.next();
        let z = self.iter.next();
        if let (Some(a), Some(b), Some(c)) = (x, y, z) {
            Some((a, b, c))
        } else {
            None
        }
    }
}

pub trait GroupBy3<T>: Iterator<Item = T> + Sized {
    fn group_by_3(self) -> GroupBy3Iterator<Self> {
        GroupBy3Iterator::new(self)
    }
}

impl<T, I: Iterator<Item = T>> GroupBy3<T> for I {}
