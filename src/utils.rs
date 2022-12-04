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
pub fn split_lines<'a>(input: &'a str) -> impl Iterator<Item = &'a str> {
    input.split('\n')
}

/// splits a string by chunks of newlines, separated by double newlines
pub fn split_lines_double<'a>(input: &'a str) -> impl Iterator<Item = Split<'a, char>> {
    input.split("\n\n").map(|chunk| chunk.split('\n'))
}

/*
/// splits a string by newlines, and parses a type out of the strings
pub fn split_and_parse_lines<'a, T>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input.split('\n').map(|s| s.parse::<T>().unwrap())
}
*/

/// splits a string by chunks of newlines, separated by double newlines, and
/// parses a type out of the strings
pub fn split_and_parse_lines_double<'a, T>(input: &'a str) -> Vec<Vec<T>>
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
