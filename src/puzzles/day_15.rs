/*
** src/puzzles/day_15.rs
** https://adventofcode.com/2022/day/15
*/

use crate::types::{Error, Point, Solution};
use crate::utils::{self, GroupBy2};

use anyhow::Result;
use regex::Regex;

use std::cmp;
use std::collections::HashSet;

#[cfg(feature = "sample")]
const TARGET_Y: i64 = 10;
#[cfg(not(feature = "sample"))]
const TARGET_Y: i64 = 2000000;

#[cfg(feature = "sample")]
const DISTRESS_BEACON_COORD_MAX: i64 = 20;
#[cfg(not(feature = "sample"))]
const DISTRESS_BEACON_COORD_MAX: i64 = 4000000;

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
    beacon_distance: i64,
}

impl Sensor {
    fn visible_range_of_row(&self, y: i64) -> Range {
        let max_y = if y < self.pos.y {
            self.pos.y - self.beacon_distance
        } else {
            self.pos.y + self.beacon_distance
        };
        let y_dist = (max_y - y).abs();
        let x_min = self.pos.x - y_dist;
        let x_max = self.pos.x + y_dist;
        Range::new(x_min, x_max)
    }
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let matches = re.captures(s).unwrap();
        let sensor_x = matches[1].parse().unwrap();
        let sensor_y = matches[2].parse().unwrap();
        let beacon_x = matches[3].parse().unwrap();
        let beacon_y = matches[4].parse().unwrap();
        let pos = Point::new(sensor_x, sensor_y);
        let closest_beacon = Point::new(beacon_x, beacon_y);
        let beacon_distance = Point::manhattan_distance(pos, closest_beacon);
        Self {
            pos,
            closest_beacon,
            beacon_distance,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(min: i64, max: i64) -> Self {
        Self { min, max }
    }

    fn size(&self) -> i64 {
        self.max - self.min
    }

    fn overlaps(&self, other: &Self) -> bool {
        (other.min >= self.min && other.min <= self.max)
            || (other.max >= self.min && other.max <= self.max)
    }

    fn try_combine(&self, other: &Self) -> (Self, Option<Self>) {
        if self.overlaps(other) {
            let min = cmp::min(self.min, other.min);
            let max = cmp::max(self.max, other.max);
            (Self::new(min, max), None)
        } else {
            (self.clone(), Some(other.clone()))
        }
    }

    fn reduction_pass(input: Vec<Self>) -> Vec<Self> {
        let n_ranges = input.len();
        let mut output = Vec::with_capacity(n_ranges);
        // attempt to reduce pairs of ranges
        // these will be sorted so they will be candidates for overlaps
        for (range_a, range_b) in input.iter().group_by_2() {
            let (range_a, maybe_range_b) = range_a.try_combine(range_b);
            output.push(range_a);
            if let Some(range_b) = maybe_range_b {
                output.push(range_b);
            }
        }
        // check if the input length was odd, the last range will be hanging
        if n_ranges % 2 != 0 {
            output.push(input[n_ranges - 1].clone());
        }
        output
    }

    fn reduce(ranges: Vec<Self>) -> Vec<Self> {
        let mut output = ranges;
        // sort the ranges to start
        output.sort_by(|a, b| a.min.cmp(&b.min));

        let mut prev_len = output.len();
        // loop until there is a single range remaining or if the pass does not
        // perform any further reductions
        loop {
            output = Self::reduction_pass(output);
            if output.len() == 1 || output.len() == prev_len {
                break;
            }
            prev_len = output.len();
        }

        output
    }
}

fn filter_sensors_by_y_view(sensors: &[Sensor], y: i64) -> impl Iterator<Item = &Sensor> {
    sensors
        .iter()
        .filter(move |s| y >= s.pos.y - s.beacon_distance && y <= s.pos.y + s.beacon_distance)
}

fn get_visible_x_range_of_row(sensors: &[Sensor], y: i64) -> Range {
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    // grab all sensors that can view the target row
    for sensor in filter_sensors_by_y_view(sensors, y) {
        let x_range = sensor.visible_range_of_row(y);
        x_min = cmp::min(x_min, x_range.min);
        x_max = cmp::max(x_max, x_range.max);
    }
    Range::new(x_min, x_max)
}

fn non_beacon_points_in_row(sensors: &[Sensor], beacons: &HashSet<Point>, y: i64) -> i64 {
    // from experimentation, this is a continuous row so iterate over the
    // sensors to find the furthest leftmost/rightmost reaches of the range
    let x_range = get_visible_x_range_of_row(sensors, y);
    // then remove any beacons from the set
    let beacons_in_row = beacons
        .iter()
        .filter(|b| b.y == y && b.x >= x_range.min && b.x <= x_range.max)
        .count() as i64;
    x_range.size() - beacons_in_row + 1
}

fn find_distress_beacon(sensors: &[Sensor]) -> Option<Point> {
    // check the visible range of each row and search for a single point gap
    for y in 0..=DISTRESS_BEACON_COORD_MAX {
        // grab all sensors that can view this row
        let row_sensors = filter_sensors_by_y_view(sensors, y).collect::<Vec<_>>();
        // there must be at least 2 sensors that can view the row in order for
        // it to contain the distress beacon
        if row_sensors.len() < 2 {
            continue;
        }
        // get the visibility ranges of the sensors across the x-axis
        let sensor_x_ranges = row_sensors
            .iter()
            .map(|s| s.visible_range_of_row(y))
            .collect::<Vec<_>>();
        // and reduce the ranges
        let sensors_x_range = Range::reduce(sensor_x_ranges);
        // we are looking for a single point of separation between 2 ranges
        // if this is found, this is the distress beacon
        if sensors_x_range.len() == 2 && sensors_x_range[1].min == sensors_x_range[0].max + 2 {
            return Some(Point::new(sensors_x_range[0].max + 1, y));
        }
    }
    // the distress beacon was not found
    None
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the sensors
    let sensors = utils::split_lines(&input)
        .map(Sensor::from)
        .collect::<Vec<_>>();
    // also gather all beacons into a set
    let beacons = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .collect::<HashSet<_>>();

    // part 1: Consult the report from the sensors you just deployed. In the
    // row where y=2000000, how many positions cannot contain a beacon?
    let points = non_beacon_points_in_row(&sensors, &beacons, TARGET_Y);
    solution.set_part_1(points);

    // part 2: Find the only possible position for the distress beacon. What is
    // its tuning frequency?
    let distress_beacon = find_distress_beacon(&sensors).ok_or(Error::NoSolution)?;
    let tuning_frequency = (distress_beacon.x * 4000000) + distress_beacon.y;
    solution.set_part_2(tuning_frequency);

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_ranges() {
        let input = vec![
            Range::new(1, 3),
            Range::new(2, 4),
            Range::new(3, 5),
            Range::new(4, 6),
        ];
        let output = Range::reduce(input);
        assert_eq!(output.len(), 1);
        let range = &output[0];
        assert_eq!(range.min, 1);
        assert_eq!(range.max, 6);

        let input = vec![
            Range::new(2, 2),
            Range::new(11, 13),
            Range::new(3, 13),
            Range::new(-3, 3),
            Range::new(15, 25),
            Range::new(15, 17),
        ];
        let output = Range::reduce(input);
        assert_eq!(output.len(), 2);
        let range_a = &output[0];
        assert_eq!(range_a.min, -3);
        assert_eq!(range_a.max, 13);
        let range_b = &output[1];
        assert_eq!(range_b.min, 15);
        assert_eq!(range_b.max, 25);
    }

    #[test]
    fn reduce_ranges_disjoint() {
        let a = Range::new(1, 4);
        let b = Range::new(10, 12);
        let input = vec![a.clone(), b.clone()];
        let output = Range::reduce(input);
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], a);
        assert_eq!(output[1], b);
    }
}
