/*
** src/puzzles/day_9.rs
** https://adventofcode.com/2022/day/9
*/

use crate::types::{Point, Solution};
use crate::utils;

use anyhow::Result;
use log::debug;

use std::collections::HashSet;

const N_KNOTS: usize = 10;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    length: i64,
}

impl From<&str> for Motion {
    fn from(s: &str) -> Self {
        let direction = Direction::from(s.chars().next().unwrap());
        let length = s[2..].parse().unwrap();
        Self { direction, length }
    }
}

struct Rope {
    head: Point,
    tail: Point,
    tail_positions: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Point::origin(),
            tail: Point::origin(),
            tail_positions: HashSet::new(),
        }
    }

    fn ends_adjacent(&self) -> bool {
        (self.head.x - self.tail.x).abs() <= 1 && (self.head.y - self.tail.y).abs() <= 1
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }
    }

    fn move_tail(&mut self) {
        // no motion necessary if the head and tail are adjacent
        if !self.ends_adjacent() {
            let dx = self.head.x - self.tail.x;
            let dy = self.head.y - self.tail.y;
            // if the head is 2 steps directly up/down/left/right from the tail
            // it must also move 1 step in that direction; otherwise, the tail
            // moves 1 step diagonally
            self.tail.x += dx.signum();
            self.tail.y += dy.signum();
        }
    }

    fn make_move(&mut self, motion: &Motion) {
        debug!("motion: {:?}", motion);
        for _ in 0..motion.length {
            self.move_head(&motion.direction);
            debug!("head @ {} tail @ {}", self.head, self.tail);
            self.move_tail();
            debug!("head @ {} tail @ {}", self.head, self.tail);
            // track the new tail position
            self.tail_positions.insert(self.tail);
        }
    }
}

struct KnottedRope {
    knots: [Point; N_KNOTS],
    tail_positions: HashSet<Point>,
}

impl KnottedRope {
    fn new() -> Self {
        Self {
            knots: [Point::origin(); N_KNOTS],
            tail_positions: HashSet::new(),
        }
    }

    fn knots_adjacent(&self, i: usize, j: usize) -> bool {
        let a = self.knots[i];
        let b = self.knots[j];
        (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Left => self.knots[0].x -= 1,
            Direction::Right => self.knots[0].x += 1,
        }
    }

    fn move_knot(&mut self, index: usize) {
        // no motion necessary if the head and tail are adjacent
        if !self.knots_adjacent(index - 1, index) {
            let dx = self.knots[index - 1].x - self.knots[index].x;
            let dy = self.knots[index - 1].y - self.knots[index].y;
            // if the head is 2 steps directly up/down/left/right from the tail
            // it must also move 1 step in that direction; otherwise, the tail
            // moves 1 step diagonally
            self.knots[index].x += dx.signum();
            self.knots[index].y += dy.signum();
        }
    }

    fn make_move(&mut self, motion: &Motion) {
        debug!("motion: {:?}", motion);
        for _ in 0..motion.length {
            self.move_head(&motion.direction);
            for i in 1..N_KNOTS {
                self.move_knot(i);
            }
            // track the new tail position
            self.tail_positions.insert(self.knots[N_KNOTS - 1]);
        }
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the motions
    let motions = utils::split_lines(&input)
        .map(Motion::from)
        .collect::<Vec<_>>();

    // part 1: Simulate your complete hypothetical series of motions. How many
    // positions does the tail of the rope visit at least once?
    let mut rope = Rope::new();
    for motion in motions.iter() {
        rope.make_move(motion);
    }
    let tail_positions = rope.tail_positions.len();
    solution.set_part_1(tail_positions);

    // part 2: Simulate your complete series of motions on a larger rope with
    // ten knots. How many positions does the tail of the rope visit at least
    // once?
    let mut knotted_rope = KnottedRope::new();
    for motion in motions.iter() {
        knotted_rope.make_move(motion);
    }
    let tail_positions = knotted_rope.tail_positions.len();
    solution.set_part_2(tail_positions);

    Ok(solution)
}
