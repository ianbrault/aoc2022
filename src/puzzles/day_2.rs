/*
** src/puzzles/day_2.rs
** https://adventofcode.com/2022/day/2
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;

/// rock/paper/scissors move
#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_result(opponent_move: &Self, result: &GameResult) -> Self {
        match result {
            GameResult::Draw => opponent_move.clone(),
            GameResult::Win => match opponent_move {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            GameResult::Loss => match opponent_move {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn get(opponent_move: &Move, player_move: &Move) -> Self {
        match (opponent_move, player_move) {
            (Move::Rock, Move::Rock) => Self::Draw,
            (Move::Rock, Move::Paper) => Self::Win,
            (Move::Rock, Move::Scissors) => Self::Loss,
            (Move::Paper, Move::Rock) => Self::Loss,
            (Move::Paper, Move::Paper) => Self::Draw,
            (Move::Paper, Move::Scissors) => Self::Win,
            (Move::Scissors, Move::Rock) => Self::Win,
            (Move::Scissors, Move::Paper) => Self::Loss,
            (Move::Scissors, Move::Scissors) => Self::Draw,
        }
    }

    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

impl From<char> for GameResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

struct Game {
    player_move: Move,
    result: GameResult,
}

impl Game {
    fn from_str_with_move(s: &str) -> Self {
        let opponent_move = Move::from(utils::nchar(s, 0));
        let player_move = Move::from(utils::nchar(s, 2));
        let result = GameResult::get(&opponent_move, &player_move);
        Self {
            player_move,
            result,
        }
    }

    fn from_str_with_result(s: &str) -> Self {
        let opponent_move = Move::from(utils::nchar(s, 0));
        let result = GameResult::from(utils::nchar(s, 2));
        let player_move = Move::from_result(&opponent_move, &result);
        Self {
            player_move,
            result,
        }
    }

    fn score(&self) -> u64 {
        self.player_move.score() + self.result.score()
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse into games with the second column being the player's move
    let games_with_move = utils::split_lines(&input)
        .map(|s| Game::from_str_with_move(s))
        .collect::<Vec<_>>();
    // parse into games with the second column being the result
    let games_with_result = utils::split_lines(&input)
        .map(|s| Game::from_str_with_result(s))
        .collect::<Vec<_>>();

    // part 1: What would your total score be if everything goes exactly
    // according to your strategy guide?
    let score_part_1 = games_with_move.iter().map(|game| game.score()).sum::<u64>();
    solution.set_part_1(score_part_1);

    // part 2: Following the Elf's instructions for the second column, what
    // would your total score be if everything goes exactly according to your
    // strategy guide?
    let score_part_2 = games_with_result
        .iter()
        .map(|game| game.score())
        .sum::<u64>();
    solution.set_part_2(score_part_2);

    Ok(solution)
}
