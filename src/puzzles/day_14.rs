/*
** src/puzzles/day_14.rs
** https://adventofcode.com/2022/day/14
*/

use crate::types::{Point, Solution};
use crate::utils;

use anyhow::Result;
use log::debug;

use std::cmp;
use std::collections::HashMap;

const FLOOR_MARGIN: i64 = 256;

struct RockPath {
    points: Vec<Point>,
}

impl From<&str> for RockPath {
    fn from(s: &str) -> Self {
        let mut points = Vec::new();
        for point_str in s.split(" -> ") {
            let sep = point_str.chars().position(|c| c == ',').unwrap();
            let x = point_str[..sep].parse().unwrap();
            let y = point_str[(sep + 1)..].parse().unwrap();
            points.push(Point::new(x, y));
        }
        Self { points }
    }
}

#[derive(Clone, PartialEq)]
enum Material {
    Rock,
    Sand,
}

#[derive(PartialEq)]
enum SandState {
    NotSpawned,
    Falling,
    AtRest,
    InTheVoid,
}

struct CaveState {
    // maps positions in the cave to the material that occupies them
    state: HashMap<Point, Material>,
    sand: Option<Point>,
    sand_state: SandState,
    lowest_rock: i64,
    leftmost_rock: i64,
    rightmost_rock: i64,
}

impl CaveState {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
            sand: None,
            sand_state: SandState::NotSpawned,
            lowest_rock: 0,
            leftmost_rock: 0,
            rightmost_rock: 0,
        }
    }

    fn set_lowest_rock(&mut self) {
        let (lowest, _) = self
            .state
            .iter()
            .filter(|(_, m)| m == &&Material::Rock)
            .max_by(|(pa, _), (pb, _)| pa.y.cmp(&pb.y))
            .unwrap();
        self.lowest_rock = lowest.y;
    }

    fn set_leftmost_rock(&mut self) {
        let (leftmost, _) = self
            .state
            .iter()
            .filter(|(_, m)| m == &&Material::Rock)
            .min_by(|(pa, _), (pb, _)| pa.x.cmp(&pb.x))
            .unwrap();
        self.leftmost_rock = leftmost.x;
    }

    fn set_rightmost_rock(&mut self) {
        let (rightmost, _) = self
            .state
            .iter()
            .filter(|(_, m)| m == &&Material::Rock)
            .max_by(|(pa, _), (pb, _)| pa.x.cmp(&pb.x))
            .unwrap();
        self.rightmost_rock = rightmost.x;
    }

    fn add_rock_path(&mut self, path: RockPath) {
        for i in 0..(path.points.len() - 1) {
            let pa = path.points[i];
            let pb = path.points[i + 1];
            // check if the line is horizontal or vertical
            if pa.x == pb.x {
                let y0 = cmp::min(pa.y, pb.y);
                let y1 = cmp::max(pa.y, pb.y);
                for y in y0..=y1 {
                    let p = Point::new(pa.x, y);
                    self.state.insert(p, Material::Rock);
                }
            } else if pa.y == pb.y {
                let x0 = cmp::min(pa.x, pb.x);
                let x1 = cmp::max(pa.x, pb.x);
                for x in x0..=x1 {
                    let p = Point::new(x, pa.y);
                    self.state.insert(p, Material::Rock);
                }
            }
        }
        // set the lowest/leftmost/rightmost point of rock
        self.set_lowest_rock();
        self.set_leftmost_rock();
        self.set_rightmost_rock();
    }

    fn sand_origin() -> Point {
        Point::new(500, 0)
    }

    fn spawn_sand(&mut self) {
        self.sand = Some(Self::sand_origin());
        self.sand_state = SandState::Falling;
    }

    fn is_air(&self, point: &Point) -> bool {
        !self.state.contains_key(point)
    }

    fn move_sand(&mut self) {
        if let Some(point) = self.sand {
            let below = Point::new(point.x, point.y + 1);
            let diag_left = Point::new(point.x - 1, point.y + 1);
            let diag_right = Point::new(point.x + 1, point.y + 1);
            // check if the sand can fall downwards 1 step, or diagonally left,
            // or diagonally right; otherwise, it will be at rest
            if self.is_air(&below) {
                self.sand = Some(below);
            } else if self.is_air(&diag_left) {
                self.sand = Some(below);
                self.sand = Some(diag_left);
            } else if self.is_air(&diag_right) {
                self.sand = Some(diag_right);
            } else {
                // sand has come to rest, add the particle to the final state
                self.state.insert(point, Material::Sand);
                self.sand_state = SandState::AtRest;
            }
            // check if the sand has fallen into the void
            if let Some(point) = self.sand {
                if point.y > self.lowest_rock {
                    debug!("sand has fallen into the void at {}", point);
                    self.sand_state = SandState::InTheVoid;
                }
            }
        } else {
            unreachable!()
        }
    }

    fn run_cycle(&mut self) {
        if self.sand_state == SandState::NotSpawned || self.sand_state == SandState::AtRest {
            // if the sand has not been spawned or the previous unit of sand is
            // at rest, spawn another unit
            self.spawn_sand();
        } else if self.sand_state == SandState::Falling {
            // otherwise the unit of sand is falling
            self.move_sand();
        } else {
            unreachable!()
        }
    }

    fn run_to_completion(&mut self) {
        let origin = Self::sand_origin();
        // run cycles until the sand has fallen into the void
        while self.sand_state != SandState::InTheVoid {
            self.run_cycle();
            // also terminate if sand has piled up to the origin point
            if self.sand_state == SandState::AtRest && self.sand == Some(origin) {
                debug!("sand has come to rest at the origin");
                break;
            }
        }
    }

    fn sand_at_rest(&self) -> usize {
        self.state
            .values()
            .filter(|v| v == &&Material::Sand)
            .count()
    }

    fn add_floor(&mut self) {
        let y = self.lowest_rock + 2;
        let x0 = self.leftmost_rock - FLOOR_MARGIN;
        let x1 = self.rightmost_rock + FLOOR_MARGIN;
        for x in x0..=x1 {
            let p = Point::new(x, y);
            self.state.insert(p, Material::Rock);
        }
        self.lowest_rock = y;
        self.leftmost_rock = x0;
        self.rightmost_rock = x1;
    }
}

impl From<Vec<RockPath>> for CaveState {
    fn from(paths: Vec<RockPath>) -> Self {
        let mut state = Self::new();
        for path in paths.into_iter() {
            state.add_rock_path(path);
        }
        state
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the rock paths
    let rock_paths = utils::split_lines(&input)
        .map(RockPath::from)
        .collect::<Vec<_>>();
    // and create the cave state object
    let mut cave_state = CaveState::from(rock_paths);

    // part 1: Using your scan, simulate the falling sand. How many units of
    // sand come to rest before sand starts flowing into the abyss below?
    cave_state.run_to_completion();
    solution.set_part_1(cave_state.sand_at_rest());

    // reset variables in between runs
    cave_state.sand = None;
    cave_state.sand_state = SandState::NotSpawned;

    // part 2: Using your scan, simulate the falling sand until the source of
    // the sand becomes blocked. How many units of sand come to rest?
    cave_state.add_floor();
    cave_state.run_to_completion();
    solution.set_part_2(cave_state.sand_at_rest());

    Ok(solution)
}
