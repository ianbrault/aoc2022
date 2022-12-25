/*
** src/puzzles/day_12.rs
** https://adventofcode.com/2022/day/12
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;
use log::debug;

use std::cmp;
use std::collections::HashSet;
use std::fmt;

#[cfg(feature = "sample")]
const WIDTH: usize = 8;
#[cfg(feature = "sample")]
const HEIGHT: usize = 5;
#[cfg(not(feature = "sample"))]
const WIDTH: usize = 101;
#[cfg(not(feature = "sample"))]
const HEIGHT: usize = 41;

#[cfg(feature = "sample")]
const BOTTOM: (usize, usize) = (0, 0);
#[cfg(feature = "sample")]
const TOP: (usize, usize) = (2, 5);
#[cfg(not(feature = "sample"))]
const BOTTOM: (usize, usize) = (20, 0);
#[cfg(not(feature = "sample"))]
const TOP: (usize, usize) = (20, 77);

const MAX_HEIGHT: i64 = 25;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coord {
    i: usize,
    j: usize,
}

impl Coord {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    fn up(&self) -> Option<Self> {
        if self.i > 0 {
            Some(Self::new(self.i - 1, self.j))
        } else {
            None
        }
    }

    fn down(&self) -> Option<Self> {
        if self.i < HEIGHT - 1 {
            Some(Self::new(self.i + 1, self.j))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Self> {
        if self.j > 0 {
            Some(Self::new(self.i, self.j - 1))
        } else {
            None
        }
    }

    fn right(&self) -> Option<Self> {
        if self.j < WIDTH - 1 {
            Some(Self::new(self.i, self.j + 1))
        } else {
            None
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(c: (usize, usize)) -> Self {
        Coord::new(c.0, c.1)
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.i, self.j)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Grid {
    grid: [[i64; WIDTH]; HEIGHT],
}

impl Grid {
    fn get(&self, coord: &Coord) -> i64 {
        self.grid[coord.i][coord.j]
    }

    fn set(&mut self, coord: &Coord, value: i64) {
        self.grid[coord.i][coord.j] = value;
    }
}

impl From<i64> for Grid {
    fn from(n: i64) -> Self {
        let grid = [[n; WIDTH]; HEIGHT];
        Self { grid }
    }
}

impl From<[[i64; WIDTH]; HEIGHT]> for Grid {
    fn from(grid: [[i64; WIDTH]; HEIGHT]) -> Self {
        Self { grid }
    }
}

fn elevation(c: char) -> i64 {
    let base = 'a' as i64;
    match c {
        'S' => 0,
        'E' => MAX_HEIGHT,
        // a is 0, z is 25
        _ => c as i64 - base,
    }
}

fn parse_heightmap(s: &str) -> Grid {
    let mut heightmap = [[0; WIDTH]; HEIGHT];
    for (i, row) in utils::split_lines(s).enumerate() {
        for (j, c) in row.chars().enumerate() {
            heightmap[i][j] = elevation(c);
        }
    }
    Grid::from(heightmap)
}

fn get_unvisited_set() -> HashSet<Coord> {
    let mut set = HashSet::new();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            set.insert(Coord::new(i, j));
        }
    }
    set
}

fn search_is_done(destination: &Coord, distances: &Grid, unvisited_set: &HashSet<Coord>) -> bool {
    // iterate until the top has been visited or the smallest tentative
    // distance in the unvisited set is infinity
    // also terminate if the unvisited set is empty
    unvisited_set.is_empty()
        || !unvisited_set.contains(destination)
        || unvisited_set
            .iter()
            .map(|p| distances.get(p))
            .min()
            .unwrap_or(i64::MAX)
            == i64::MAX
}

fn is_reachable(heightmap: &Grid, current: &Coord, destination: &Coord) -> bool {
    let height_curr = heightmap.get(current);
    let height_dest = heightmap.get(destination);
    height_curr - height_dest <= 1
}

fn unvisited_neighbors(
    point: &Coord,
    heightmap: &Grid,
    unvisited_set: &HashSet<Coord>,
) -> Vec<Coord> {
    let neighbors = vec![point.up(), point.down(), point.left(), point.right()];
    neighbors
        .into_iter()
        .flatten()
        .filter(|p| is_reachable(heightmap, point, p))
        .filter(|p| unvisited_set.contains(p))
        .collect()
}

fn next_node(unvisited_set: &HashSet<Coord>, distances: &Grid) -> Option<Coord> {
    // select the unvisited node with the smallest tentative distance
    if let Some((point, _)) = unvisited_set
        .iter()
        .map(|p| (p, distances.get(p)))
        .min_by(|(_, da), (_, db)| da.cmp(db))
    {
        Some(point.clone())
    } else {
        None
    }
}

fn dijkstra(heightmap: &Grid) -> Grid {
    let bottom = Coord::from(BOTTOM);
    let top = Coord::from(TOP);
    let mut unvisited_set = get_unvisited_set();

    // set all tentative distances to infinity and set the top to 0
    let mut distances = Grid::from(i64::MAX);
    distances.set(&top, 0);

    // start with the top
    let mut current_node = top.clone();
    // iterate until the bottom has been visited or the smallest tentative
    // distance in the unvisited set is infinity
    while !search_is_done(&bottom, &distances, &unvisited_set) {
        debug!("visiting node {}", current_node);
        let distance = distances.get(&current_node);
        // consider all unvisited neighbors
        for node in unvisited_neighbors(&current_node, heightmap, &unvisited_set).iter() {
            // calculate their tentative distance thru the current node
            let node_distance = distances.get(node);
            let new_distance = distance + 1;
            distances.set(node, cmp::min(node_distance, new_distance));
        }
        // remove the current node from the unvisited set
        unvisited_set.remove(&current_node);
        // select the unvisited node with the smallest tentative distance
        if let Some(node) = next_node(&unvisited_set, &distances) {
            current_node = node;
        }
    }

    distances
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the height-map
    let heightmap = parse_heightmap(&input);
    // and calculate the distances to the top
    let distances = dijkstra(&heightmap);

    // part 1: What is the fewest steps required to move from your current
    // position to the location that should get the best signal?
    let bottom = Coord::from(BOTTOM);
    let best_path_from_start = distances.get(&bottom);
    solution.set_part_1(best_path_from_start);

    // part 2: What is the fewest steps required to move starting from any
    // square with elevation a to the location that should get the best signal?
    let best_path_from_bottom = get_unvisited_set()
        .into_iter()
        .filter(|p| heightmap.get(p) == 0)
        .map(|p| distances.get(&p))
        .min()
        .unwrap();
    solution.set_part_2(best_path_from_bottom);

    Ok(solution)
}
