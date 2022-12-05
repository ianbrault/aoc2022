/*
** src/puzzles/day_5.rs
** https://adventofcode.com/2022/day/5
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;

const N_STACKS: usize = 9;

struct Move {
    n_crates: u8,
    from: u8,
    to: u8,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let words = s.split(' ').collect::<Vec<_>>();
        let n_crates = words[1].parse().unwrap();
        let from = words[3].parse().unwrap();
        let to = words[5].parse().unwrap();
        Self { n_crates, from, to }
    }
}

#[derive(Clone)]
struct Stacks {
    stacks: [Vec<char>; N_STACKS],
    buffer: Vec<char>,
}

impl Stacks {
    fn top(&self) -> String {
        self.stacks.iter().map(|s| s[s.len() - 1]).collect()
    }

    fn crate_mover_9000(&mut self, m: &Move) {
        let from = (m.from - 1) as usize;
        let to = (m.to - 1) as usize;
        for _ in 0..m.n_crates {
            let crate_name = self.stacks[from].pop().unwrap();
            self.stacks[to].push(crate_name);
        }
    }

    fn crate_mover_9001(&mut self, m: &Move) {
        let from = (m.from - 1) as usize;
        let to = (m.to - 1) as usize;
        // first load crates into the buffer
        for _ in 0..m.n_crates {
            let crate_name = self.stacks[from].pop().unwrap();
            self.buffer.push(crate_name);
        }
        // then drain from the buffer
        while let Some(crate_name) = self.buffer.pop() {
            self.stacks[to].push(crate_name);
        }
    }
}

impl From<&str> for Stacks {
    fn from(s: &str) -> Self {
        let mut stacks: [Vec<char>; N_STACKS] = Default::default();
        let lines = utils::split_lines(s).collect::<Vec<_>>();

        for line in lines[0..(lines.len() - 1)].iter().rev() {
            let n_cols = (line.len() + 1) / 4;
            for (col, stack) in stacks.iter_mut().enumerate().take(n_cols) {
                let i = col * 4 + 1;
                let crate_name = line[i..(i + 1)].chars().next().unwrap();
                if crate_name != ' ' {
                    stack.push(crate_name);
                }
            }
        }

        Self {
            stacks,
            buffer: Vec::new(),
        }
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the initial stacks and move list
    let (mut stacks_1, moves) = match input.split("\n\n").collect::<Vec<_>>().as_slice() {
        &[stacks_str, moves_str] => {
            let stacks = Stacks::from(stacks_str);
            let moves = utils::split_lines(moves_str)
                .map(Move::from)
                .collect::<Vec<_>>();
            (stacks, moves)
        }
        _ => unreachable!(),
    };
    // clone for part 2
    let mut stacks_2 = stacks_1.clone();

    // part 1: After the rearrangement procedure completes, what crate ends up
    // on top of each stack?
    for m in moves.iter() {
        stacks_1.crate_mover_9000(m);
    }
    solution.set_part_1(stacks_1.top());

    // part 2: Before the rearrangement process finishes, update your
    // simulation so that the Elves know where they should stand to be ready to
    // unload the final supplies. After the rearrangement procedure completes,
    // what crate ends up on top of each stack?
    for m in moves.iter() {
        stacks_2.crate_mover_9001(m);
    }
    solution.set_part_2(stacks_2.top());

    Ok(solution)
}
