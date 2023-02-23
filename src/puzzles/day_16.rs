/*
** src/puzzles/day_16.rs
** https://adventofcode.com/2022/day/16
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;
use itertools::Itertools;
use log::debug;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt;

const CHAR_BASE: u16 = 'A' as u16;
const TIME_LIMIT: u64 = 30;
const TIME_LIMIT_WITH_ELEPHANT: u64 = 26;

// there are 26 letters, this requires 5 bits per letter
// this means we need 10 bits per valve
// this ends up with 1024 options
const VALVE_BUF_SIZE: usize = 1 << 10;
// valves are connected to at most 5 other valves
const MAX_CONNECTIONS: usize = 5;

// NOTE: converted Valve to an integer-struct to avoid lifetime complications
// valves are 2-letter string identifiers: the first letter is the upper 5 bits
// and the second letter is the lower 5 bits
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Valve(u16);

impl From<&str> for Valve {
    fn from(s: &str) -> Self {
        let ca = utils::nchar(s, 0);
        let cb = utils::nchar(s, 1);
        let a = (ca as u16) - CHAR_BASE;
        let b = (cb as u16) - CHAR_BASE;
        Self(((a & 0x1F) << 5) | (b & 0x1F))
    }
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = ((self.0 >> 5) & 0x1F) + CHAR_BASE;
        let b = (self.0 & 0x1F) + CHAR_BASE;
        let ca = char::from_u32(a as u32).unwrap();
        let cb = char::from_u32(b as u32).unwrap();
        write!(f, "{}{}", ca, cb)
    }
}

// maps valve IDs to their flow rates
// this array is 8 KiB so stick it on the heap
struct FlowRates(Vec<u64>);

impl FlowRates {
    fn new() -> Self {
        let inner = vec![u64::MAX; VALVE_BUF_SIZE];
        Self(inner)
    }

    fn get(&self, vid: u16) -> u64 {
        self.0[vid as usize]
    }

    fn set(&mut self, vid: u16, value: u64) {
        self.0[vid as usize] = value;
    }
}

// maps valve IDs to the valve IDs that they are connected to
// this array is 10 KiB so stick it on the heap
struct TunnelMap(Vec<Vec<u16>>);

impl TunnelMap {
    fn new() -> Self {
        let inner = vec![vec![u16::MAX; MAX_CONNECTIONS]; VALVE_BUF_SIZE];
        Self(inner)
    }

    fn set(&mut self, vid_i: u16, vid_j: u16, value: u16) {
        self.0[vid_i as usize][vid_j as usize] = value;
    }

    fn connections(&self, vid: u16) -> impl Iterator<Item = &u16> {
        self.0[vid as usize].iter().take_while(|&&v| v != u16::MAX)
    }
}

// 2-D array that stores the distances between pairs of valve IDs
// this array is at least 1 MiB (depending on usize) so stick it on the heap
#[derive(Clone)]
struct Distances(Vec<Vec<u64>>);

impl Distances {
    fn new() -> Self {
        let inner = vec![vec![u64::MAX; VALVE_BUF_SIZE]; VALVE_BUF_SIZE];
        Self(inner)
    }

    fn get(&self, vid_a: u16, vid_b: u16) -> u64 {
        self.0[vid_a as usize][vid_b as usize]
    }

    fn set(&mut self, vid_a: u16, vid_b: u16, value: u64) {
        self.0[vid_a as usize][vid_b as usize] = value;
    }
}

struct VolcanoInfo {
    flow_rates: FlowRates,
    distances: Distances,
}

impl VolcanoInfo {
    fn new(flow_rates: FlowRates, distances: Distances) -> Self {
        Self {
            flow_rates,
            distances,
        }
    }

    fn flow_rate(&self, valve: u16) -> u64 {
        self.flow_rates.get(valve)
    }

    fn distance(&self, from: u16, to: u16) -> u64 {
        self.distances.get(from, to)
    }
}

fn parse_flow_rates(input: &str) -> FlowRates {
    debug!("parsing valve flow rates");
    let mut flow_rates = FlowRates::new();
    for line in utils::split_lines(input) {
        let valve = Valve::from(&line[6..8]);
        let flow_end = utils::find_char(line, ';').unwrap();
        let flow = line[23..flow_end].parse().unwrap();
        flow_rates.set(valve.0, flow);
    }
    flow_rates
}

fn parse_tunnel_map(input: &str) -> TunnelMap {
    debug!("parsing tunnel map");
    let mut tunnel_map = TunnelMap::new();
    for line in utils::split_lines(input) {
        let valve = Valve::from(&line[6..8]);
        let flow_end = utils::find_char(line, ';').unwrap();
        // note: valve vs. valves for plural
        let offset = if line.contains("valves") { 25 } else { 24 };
        for (i, v) in line[(flow_end + offset)..]
            .split(", ")
            .map(Valve::from)
            .enumerate()
        {
            tunnel_map.set(valve.0, i as u16, v.0);
        }
    }
    tunnel_map
}

fn add_valve_connected_nodes(
    flow_rates: &FlowRates,
    tunnel_map: &TunnelMap,
    distances: &mut Distances,
    from: u16,
    to: u16,
    prev: u16,
    distance: u64,
) {
    // look at all connected valves
    for &vid in tunnel_map.connections(to) {
        // no loopbacks
        if vid == from || vid == prev {
            continue;
        }
        // compress 0-flow nodes (except for AA)
        if flow_rates.get(vid) == 0 && vid != 0 {
            add_valve_connected_nodes(
                flow_rates,
                tunnel_map,
                distances,
                from,
                vid,
                to,
                distance + 1,
            );
        } else {
            distances.set(from, vid, distance);
        }
    }
}

fn get_valve_graph(flow_rates: &FlowRates, tunnel_map: &TunnelMap) -> Distances {
    debug!("compressing valve graph to remove 0-flow nodes");
    let mut distances = Distances::new();

    // loop thru all valves
    for (vid, &flow_rate) in flow_rates.0.iter().enumerate() {
        let vid = vid as u16;
        // skip valves with 0 flow (except for AA since it is the start node)
        if flow_rate == u64::MAX || (flow_rate == 0 && vid != 0) {
            continue;
        }
        // add the self-connection
        distances.set(vid, vid, 0);
        debug!("adding connected nodes for valve {}", Valve(vid));
        for &v in tunnel_map.connections(vid) {
            // compress 0-flow nodes (except for AA)
            if flow_rates.get(v) == 0 && v != 0 {
                add_valve_connected_nodes(flow_rates, tunnel_map, &mut distances, vid, v, vid, 2);
            } else {
                distances.set(vid, v, 1);
            }
        }
    }

    distances
}

fn floyd_warshall(distances: &mut Distances) {
    for k in 0..(VALVE_BUF_SIZE as u16) {
        for i in 0..(VALVE_BUF_SIZE as u16) {
            let dik = distances.get(i, k);
            if dik == u64::MAX {
                continue;
            }
            for j in 0..(VALVE_BUF_SIZE as u16) {
                let dij = distances.get(i, j);
                let dkj = distances.get(k, j);
                if dkj == u64::MAX {
                    continue;
                }
                if dij > dik + dkj {
                    distances.set(i, j, dik + dkj);
                }
            }
        }
    }
}

fn valve_heuristic(info: &VolcanoInfo, target: u16, from: u16) -> i64 {
    info.flow_rate(target) as i64 - info.distance(from, target) as i64
}

fn find_max_pressure_release_rec(
    info: &VolcanoInfo,
    mut open_valves: HashMap<u16, bool>,
    valve: u16,
    mut time: u64,
    mut flow_rate: u64,
    mut flow_volume: u64,
    time_limit: u64,
) -> u64 {
    // if this is not the start valve AA, open the valve
    if valve != 0 {
        time += 1;
        flow_volume += flow_rate;
        flow_rate += info.flow_rate(valve);
        open_valves.insert(valve, true);
        // check if this has reached the time limit
        if time == time_limit {
            debug!(
                "time limit reached with flow_rate={} flow_volume={}",
                flow_rate, flow_volume,
            );
            return flow_volume;
        }
    }

    // check if all valves are open
    if open_valves.values().all(|&open| open) {
        // extrapolate the current flow to the remaining time
        let dt = time_limit - time + 1;
        flow_volume += dt * flow_rate;
        debug!(
            "all valves are open with time={} dt={} flow_rate={} flow_volume={}",
            time, dt, flow_rate, flow_volume,
        );
        return flow_volume;
    }

    // now consider all unopened valves, using a heuristic that combines their
    // flow rate with the distance to reach them
    let mut candidates = open_valves
        .iter()
        .filter(|(_, &is_open)| !is_open)
        .map(|(&vid, _)| vid)
        .collect::<Vec<_>>();
    candidates.sort_by(|&a, &b| {
        let ha = valve_heuristic(info, a, valve);
        let hb = valve_heuristic(info, b, valve);
        ha.cmp(&hb)
    });
    let mut results = Vec::new();
    for vid in candidates.into_iter() {
        let distance = info.distance(valve, vid);
        // visit the next valve, advancing time and flow accordingly
        let t = time + distance;
        // check if the new time is beyond the time limit
        if t >= time_limit {
            let dt = time_limit - time + 1;
            let new_flow_volume = flow_volume + (flow_rate * dt);
            debug!(
                "time limit reached with flow_rate={} flow_volume={}",
                flow_rate, new_flow_volume,
            );
            results.push(new_flow_volume);
        } else {
            let new_flow_volume = flow_volume + (flow_rate * distance);
            let res = find_max_pressure_release_rec(
                info,
                open_valves.clone(),
                vid,
                t,
                flow_rate,
                new_flow_volume,
                time_limit,
            );
            results.push(res);
        }
    }

    results.into_iter().max().unwrap()
}

fn find_max_pressure_release(info: &VolcanoInfo) -> u64 {
    let mut open_valves = info
        .flow_rates
        .0
        .iter()
        .enumerate()
        .filter(|(_, &flow)| flow != 0 && flow != u64::MAX)
        .map(|(vid, _)| (vid as u16, false))
        .collect::<HashMap<_, _>>();
    open_valves.insert(0, true);

    find_max_pressure_release_rec(info, open_valves, 0, 1, 0, 0, TIME_LIMIT)
}

fn generate_valve_partitions(info: &VolcanoInfo) -> Vec<(HashSet<u16>, HashSet<u16>)> {
    // first gather the non-zero flow valves
    let mut valves = info
        .flow_rates
        .0
        .iter()
        .enumerate()
        .filter(|(_, &flow)| flow != 0 && flow != u64::MAX)
        .map(|(vid, _)| vid as u16)
        .collect::<Vec<_>>();
    valves.sort();
    let valves_set = HashSet::<_>::from_iter(valves.clone().into_iter());
    let n_valves = valves.len();

    // generate combinations of each partition size
    let mut partitions = Vec::with_capacity(n_valves * n_valves);
    for n in 0..=n_valves {
        if n == 0 {
            let a = HashSet::new();
            let b = valves_set.clone();
            partitions.push((a, b));
        } else if n == n_valves {
            let a = valves_set.clone();
            let b = HashSet::new();
            partitions.push((a, b));
        } else {
            for combo in valves.clone().into_iter().combinations(n) {
                let a = HashSet::<_>::from_iter(combo.into_iter());
                let b = valves_set.difference(&a).copied().collect();
                partitions.push((a, b));
            }
        }
    }

    partitions
}

fn count_valves(info: &VolcanoInfo) -> usize {
    info.flow_rates
        .0
        .iter()
        .enumerate()
        .filter(|(_, &flow)| flow != 0 && flow != u64::MAX)
        .count()
}

fn get_max_pressure_release_from_valve_set(info: &VolcanoInfo, valve_set: HashSet<u16>) -> u64 {
    let mut open_valves = valve_set
        .into_iter()
        .map(|vid| (vid, false))
        .collect::<HashMap<_, _>>();
    open_valves.insert(0, true);

    find_max_pressure_release_rec(info, open_valves, 0, 1, 0, 0, TIME_LIMIT_WITH_ELEPHANT)
}

fn find_max_pressure_release_with_elephant(info: &VolcanoInfo) -> u64 {
    // brute force: generate all partitions of valves and check which
    // permutation produces the maximum flow
    let valve_sets = generate_valve_partitions(info);
    debug!("generated {} valve partitions", valve_sets.len());
    // filter out any partition in which either set has fewer than 25% of all
    let cutoff = count_valves(info) / 4;
    let valve_sets_filtered = valve_sets
        .into_iter()
        .filter(|(a, b)| a.len() >= cutoff && b.len() >= cutoff)
        .collect::<Vec<_>>();
    debug!(
        "filtered down to {} valve partitions",
        valve_sets_filtered.len()
    );

    let mut max_pressure = 0;
    for (human_valves, elephant_valves) in valve_sets_filtered.into_iter() {
        let human_pressure = get_max_pressure_release_from_valve_set(info, human_valves);
        let elephant_pressure = get_max_pressure_release_from_valve_set(info, elephant_valves);
        max_pressure = cmp::max(max_pressure, human_pressure + elephant_pressure);
    }

    max_pressure
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the valve flow rates and the tunnel map
    let flow_rates = parse_flow_rates(&input);
    let tunnel_map = parse_tunnel_map(&input);
    // then calculate the distances between valves, first compressing the graph
    // to remove the zero-flow nodes
    let mut distances = get_valve_graph(&flow_rates, &tunnel_map);
    floyd_warshall(&mut distances);

    // package the info into a single struct
    let info = VolcanoInfo::new(flow_rates, distances);

    // part 1: Work out the steps to release the most pressure in 30 minutes.
    // What is the most pressure you can release?
    let max_pressure = find_max_pressure_release(&info);
    solution.set_part_1(max_pressure);

    // part 2: With you and an elephant working together for 26 minutes, what
    // is the most pressure you could release?
    let max_pressure_w_elephant = find_max_pressure_release_with_elephant(&info);
    solution.set_part_2(max_pressure_w_elephant);

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve_from_str() {
        let input = "AA";
        let output = Valve::from(input);
        assert_eq!(output.0, 0);

        let input = "AC";
        let output = Valve::from(input);
        assert_eq!(output.0, 2);

        let input = "DA";
        let output = Valve::from(input);
        assert_eq!(output.0, 3 << 5);

        let input = "FC";
        let output = Valve::from(input);
        assert_eq!(output.0, (5 << 5) | 2);
    }

    #[test]
    fn test_valve_to_str() {
        let input = Valve(0);
        let output = format!("{}", input);
        assert_eq!(output.as_str(), "AA");

        let input = Valve(2);
        let output = format!("{}", input);
        assert_eq!(output.as_str(), "AC");

        let input = Valve(3 << 5);
        let output = format!("{}", input);
        assert_eq!(output.as_str(), "DA");

        let input = Valve((5 << 5) | 2);
        let output = format!("{}", input);
        assert_eq!(output.as_str(), "FC");
    }
}
