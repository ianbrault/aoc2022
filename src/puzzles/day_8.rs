/*
** src/puzzles/day_8.rs
** https://adventofcode.com/2022/day/8
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;
use log::debug;

use std::cmp;

#[cfg(feature = "sample")]
const SIZE: usize = 5;
#[cfg(not(feature = "sample"))]
const SIZE: usize = 99;

const fn is_exterior(row: usize, col: usize) -> bool {
    row == 0 || col == 0 || row == SIZE - 1 || col == SIZE - 1
}

fn is_visible_up(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> bool {
    let height = heights[row][col];
    (0..row).all(|i| heights[i][col] < height)
}

fn is_visible_down(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> bool {
    let height = heights[row][col];
    ((row + 1)..SIZE).all(|i| heights[i][col] < height)
}

fn is_visible_left(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> bool {
    let height = heights[row][col];
    (0..col).all(|i| heights[row][i] < height)
}

fn is_visible_right(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> bool {
    let height = heights[row][col];
    ((col + 1)..SIZE).all(|i| heights[row][i] < height)
}

fn is_visible(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> bool {
    // check left/right first for better cache performance
    is_exterior(row, col)
        || is_visible_left(heights, row, col)
        || is_visible_right(heights, row, col)
        || is_visible_up(heights, row, col)
        || is_visible_down(heights, row, col)
}

fn viewing_distance_up(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> u64 {
    let height = heights[row][col];
    let mut dist = 1;
    let mut i = row as i64 - 1;
    while i > 0 && heights[i as usize][col] < height {
        dist += 1;
        i -= 1;
    }
    dist
}

fn viewing_distance_down(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> u64 {
    let height = heights[row][col];
    let mut dist = 1;
    let mut i = row as i64 + 1;
    while (i as usize) < SIZE - 1 && heights[i as usize][col] < height {
        dist += 1;
        i += 1;
    }
    dist
}

fn viewing_distance_left(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> u64 {
    let height = heights[row][col];
    let mut dist = 1;
    let mut j = col as i64 - 1;
    while j > 0 && heights[row][j as usize] < height {
        dist += 1;
        j -= 1;
    }
    dist
}

fn viewing_distance_right(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> u64 {
    let height = heights[row][col];
    let mut dist = 1;
    let mut j = col as i64 + 1;
    while (j as usize) < SIZE - 1 && heights[row][j as usize] < height {
        dist += 1;
        j += 1;
    }
    dist
}

fn scenic_score(heights: &[[u32; SIZE]; SIZE], row: usize, col: usize) -> u64 {
    if is_exterior(row, col) {
        debug!("tree ({},{}) is exterior with scenic score 0", row, col);
        0
    } else {
        // check left/right first for better cache performance
        let left = viewing_distance_left(heights, row, col);
        debug!("tree ({},{}) has left viewing distance {}", row, col, left);
        let right = viewing_distance_right(heights, row, col);
        debug!(
            "tree ({},{}) has right viewing distance {}",
            row, col, right
        );
        let up = viewing_distance_up(heights, row, col);
        debug!("tree ({},{}) has up viewing distance {}", row, col, up);
        let down = viewing_distance_down(heights, row, col);
        debug!("tree ({},{}) has down viewing distance {}", row, col, down);
        left * right * up * down
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    let mut tree_heights = [[0; SIZE]; SIZE];
    // parse the tree hights as a 2D array
    for (i, line) in utils::split_lines(&input).enumerate() {
        for (j, height) in line.chars().enumerate() {
            tree_heights[i][j] = height.to_digit(10).unwrap();
        }
    }

    // part 1: Consider your map; how many trees are visible from outside the
    // grid?
    let mut n_visible = 0u64;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if is_visible(&tree_heights, i, j) {
                n_visible += 1;
            }
        }
    }
    solution.set_part_1(n_visible);

    // part 2: Consider each tree on your map. What is the highest scenic score
    // possible for any tree?
    let mut most_scenic = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            let score = scenic_score(&tree_heights, i, j);
            most_scenic = cmp::max(most_scenic, score);
        }
    }
    solution.set_part_2(most_scenic);

    Ok(solution)
}
