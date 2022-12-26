/*
** src/puzzles/day_13.rs
** https://adventofcode.com/2022/day/13
*/

use crate::types::Solution;
use crate::utils::{self, GroupBy2};

use anyhow::Result;
use log::debug;

use std::cmp;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    Integer(u8),
    List(Vec<PacketData>),
}

impl PacketData {
    fn find_list_end(s: &str) -> usize {
        let mut n_open = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '[' {
                n_open += 1;
            } else if c == ']' {
                n_open -= 1;
                if n_open == 0 {
                    return i;
                }
            }
        }
        s.len()
    }

    fn parse_list(s: &str) -> Self {
        let mut items = Vec::new();
        // ignore the opening and closing brackets
        let s = &s[1..(s.len() - 1)];
        let chars = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < s.len() {
            let c = chars[i];
            if c == ',' {
                // skip the comma separators
                i += 1;
            } else if c == '[' {
                // parse a sub-list if one is found
                let end = Self::find_list_end(&s[i..]) + i;
                let sublist = Self::parse_list(&s[i..=end]);
                items.push(sublist);
                i = end + 1;
            } else {
                // otherwise, parse the number
                // NOTE: these are no larger than 10
                if i + 1 < s.len() && chars[i + 1].is_ascii_digit() {
                    let n = s[i..(i + 2)].parse().unwrap();
                    items.push(Self::Integer(n));
                    i += 2;
                } else {
                    let n = c.to_digit(10).unwrap() as u8;
                    items.push(Self::Integer(n));
                    i += 1;
                };
            }
        }

        Self::List(items)
    }

    fn divider_packets() -> [Self; 2] {
        [
            Self::List(vec![Self::List(vec![Self::Integer(2)])]),
            Self::List(vec![Self::List(vec![Self::Integer(6)])]),
        ]
    }

    fn make_list(&self) -> Self {
        match self {
            int @ Self::Integer(_) => Self::List(vec![int.clone()]),
            list @ Self::List(_) => list.clone(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::List(list) => list.len(),
            // pre-condition: must be called on a list
            Self::Integer(_) => unreachable!(),
        }
    }

    fn item_at(&self, i: usize) -> &PacketData {
        match self {
            Self::List(list) => &list[i],
            // pre-condition: must be called on a list
            Self::Integer(_) => unreachable!(),
        }
    }
}

impl cmp::PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let left = &self;
        let right = other;
        debug!("comparing lists {} vs. {}", left, right);
        // compare element-by-element
        let bound = cmp::min(left.len(), right.len());
        for i in 0..bound {
            let left_item = left.item_at(i);
            let right_item = right.item_at(i);
            debug!("comparing items {} vs. {}", left_item, right_item);
            match (left_item, right_item) {
                // if both values are integers, the lower integer should come
                // first; if the left integer is lower than the right, the inputs
                // are in the right order; if the left integer is higher than the
                // right, the inputs are not in the right order; otherwise, the
                // inputs are the same integer, continue on
                (PacketData::Integer(left), PacketData::Integer(right)) =>
                {
                    #[allow(clippy::comparison_chain)]
                    if left < right {
                        debug!("left is lower, inputs are in the right order");
                        return Some(cmp::Ordering::Less);
                    } else if left > right {
                        debug!("left is higher, inputs are NOT in the right order");
                        return Some(cmp::Ordering::Greater);
                    } else {
                        debug!("left and right are the same, continuing on");
                    }
                }
                // if both values are lists, compare the first value of each list,
                // then the second, and so on; if the left list runs out of items
                // first, the inputs are in the right order; if the right list runs
                // out of items first, the inputs are not in the right order; if
                // the lists are the same length and no comparison makes a decision
                // about the order, continue on
                (left @ PacketData::List(_), right @ PacketData::List(_)) => {
                    let result = left.partial_cmp(right);
                    if let Some(cmp::Ordering::Less) = result {
                        debug!("left list compares lower, inputs are in the right order");
                        return Some(cmp::Ordering::Less);
                    } else if let Some(cmp::Ordering::Greater) = result {
                        debug!("left list compares higher, inputs are NOT in the right order");
                        return Some(cmp::Ordering::Greater);
                    } else {
                        debug!("left and right lists are the same, continuing on");
                    }
                }
                // if exactly one value is an integer, convert it to a list which
                // contains that integer as its only value, then retry comparison
                (left @ PacketData::Integer(_), right @ PacketData::List(_)) => {
                    debug!("converting {} to a list and retrying", left);
                    let left = left.make_list();
                    let result = left.partial_cmp(right);
                    if result.is_some() {
                        return result;
                    }
                }
                (left @ PacketData::List(_), right @ PacketData::Integer(_)) => {
                    debug!("converting {} to a list and retrying", right);
                    let right = right.make_list();
                    let result = left.partial_cmp(&right);
                    if result.is_some() {
                        return result;
                    }
                }
            }
        }
        // check if one list has ran out of items; if the left list runs out of
        // items first, the inputs are in the right order; if the right list runs
        // out of items first, the inputs are not in the right order
        if right.len() > bound {
            debug!("left list ran out of items first, inputs are in the right order");
            Some(cmp::Ordering::Less)
        } else if left.len() > bound {
            debug!("right list ran out of items first, inputs are NOT in the right order");
            Some(cmp::Ordering::Greater)
        } else {
            debug!("no decision could be made");
            None
        }
    }
}

impl cmp::Ord for PacketData {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for PacketData {
    fn from(s: &str) -> Self {
        Self::parse_list(s)
    }
}

impl fmt::Display for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::List(list) => {
                let mut parts = Vec::new();
                for item in list.iter() {
                    parts.push(format!("{}", item));
                }
                write!(f, "[{}]", parts.join(","))
            }
        }
    }
}

fn parse_packets(input: &str) -> Vec<PacketData> {
    let mut packets = Vec::new();
    for chunk in utils::split_lines_double(input) {
        for line in chunk {
            packets.push(PacketData::from(line));
        }
    }
    packets
}

/*
fn lists_in_order(left: &PacketData, right: &PacketData) -> Option<bool> {
    debug!("comparing lists {} vs. {}", left, right);
    // compare element-by-element
    let bound = cmp::min(left.len(), right.len());
    for i in 0..bound {
        let left_item = left.item_at(i);
        let right_item = right.item_at(i);
        debug!("comparing items {} vs. {}", left_item, right_item);
        match (left_item.as_ref(), right_item.as_ref()) {
            // if both values are integers, the lower integer should come
            // first; if the left integer is lower than the right, the inputs
            // are in the right order; if the left integer is higher than the
            // right, the inputs are not in the right order; otherwise, the
            // inputs are the same integer, continue on
            (PacketData::Integer(left), PacketData::Integer(right)) => {
                if left < right {
                    debug!("left is lower, inputs are in the right order");
                    return Some(true);
                } else if left > right {
                    debug!("left is higher, inputs are NOT in the right order");
                    return Some(false);
                } else {
                    debug!("left and right are the same, continuing on");
                }
            }
            // if both values are lists, compare the first value of each list,
            // then the second, and so on; if the left list runs out of items
            // first, the inputs are in the right order; if the right list runs
            // out of items first, the inputs are not in the right order; if
            // the lists are the same length and no comparison makes a decision
            // about the order, continue on
            (left @ PacketData::List(_), right @ PacketData::List(_)) => {
                let result = lists_in_order(&left, &right);
                if let Some(true) = result {
                    debug!("left list compares lower, inputs are in the right order");
                    return Some(true);
                } else if let Some(false) = result {
                    debug!("left list compares higher, inputs are NOT in the right order");
                    return Some(false);
                } else {
                    debug!("left and right lists are the same, continuing on");
                }
            }
            // if exactly one value is an integer, convert it to a list which
            // contains that integer as its only value, then retry comparison
            (left @ PacketData::Integer(_), right @ PacketData::List(_)) => {
                debug!("converting {} to a list and retrying", left);
                let left = left.make_list();
                let result = lists_in_order(&left, &right);
                if result.is_some() {
                    return result;
                }
            }
            (left @ PacketData::List(_), right @ PacketData::Integer(_)) => {
                debug!("converting {} to a list and retrying", right);
                let right = right.make_list();
                let result = lists_in_order(&left, &right);
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    // check if one list has ran out of items; if the left list runs out of
    // items first, the inputs are in the right order; if the right list runs
    // out of items first, the inputs are not in the right order
    if right.len() > bound {
        debug!("left list ran out of items first, inputs are in the right order");
        Some(true)
    } else if left.len() > bound {
        debug!("right list ran out of items first, inputs are NOT in the right order");
        Some(false)
    } else {
        debug!("no decision could be made");
        None
    }
}
*/

fn pair_in_order(pair: (&PacketData, &PacketData)) -> bool {
    let (left, right) = pair;
    // lists_in_order(left, right).unwrap()
    match left.partial_cmp(right) {
        Some(cmp::Ordering::Less) => true,
        Some(cmp::Ordering::Greater) => false,
        _ => unreachable!(),
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the packets
    let mut packets = parse_packets(&input);

    // part 1: Determine which pairs of packets are already in the right order.
    // What is the sum of the indices of those pairs?
    let sum = packets
        .iter()
        .group_by_2()
        .enumerate()
        .filter(|(_, pair)| pair_in_order(*pair))
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    solution.set_part_1(sum);

    // part 2: Organize all of the packets into the correct order. What is the
    // decoder key for the distress signal?
    let divider_packets = PacketData::divider_packets();
    // add the additional divider packets
    debug!(
        "adding divider packets {} and {}",
        divider_packets[0], divider_packets[1]
    );
    packets.extend_from_slice(&divider_packets);
    // sort so that the packets are in the correct order
    packets.sort();
    debug!("sorted packets:");
    for packet in packets.iter() {
        debug!("{}", packet);
    }
    // find where the divider packets ended up
    let idx_a = packets
        .iter()
        .position(|p| p == &divider_packets[0])
        .unwrap()
        + 1;
    let idx_b = packets
        .iter()
        .position(|p| p == &divider_packets[1])
        .unwrap()
        + 1;
    let decoder_key = idx_a * idx_b;
    solution.set_part_2(decoder_key);

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_flat_list() {
        let input = "[1,10,2,10]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![
            PacketData::Integer(1),
            PacketData::Integer(10),
            PacketData::Integer(2),
            PacketData::Integer(10),
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    fn parse_list_single_item() {
        let input = "[1]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![PacketData::Integer(1)]);
        assert_eq!(output, expected);

        let input = "[10]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![PacketData::Integer(10)]);
        assert_eq!(output, expected);
    }

    #[test]
    fn parse_list_empty() {
        let input = "[]";
        let output = PacketData::from(input);
        let expected = PacketData::List(Vec::new());
        assert_eq!(output, expected);

        let input = "[[[]]]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![PacketData::List(vec![PacketData::List(Vec::new())])]);
        assert_eq!(output, expected);
    }

    #[test]
    fn parse_list_sublist() {
        let input = "[[1],[2,3,4]]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![
            PacketData::List(vec![PacketData::Integer(1)]),
            PacketData::List(vec![
                PacketData::Integer(2),
                PacketData::Integer(3),
                PacketData::Integer(4),
            ]),
        ]);
        assert_eq!(output, expected);

        let input = "[[4,4],4,4]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![
            PacketData::List(vec![PacketData::Integer(4), PacketData::Integer(4)]),
            PacketData::Integer(4),
            PacketData::Integer(4),
        ]);
        assert_eq!(output, expected);

        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let output = PacketData::from(input);
        let expected = PacketData::List(vec![
            PacketData::Integer(1),
            PacketData::List(vec![
                PacketData::Integer(2),
                PacketData::List(vec![
                    PacketData::Integer(3),
                    PacketData::List(vec![
                        PacketData::Integer(4),
                        PacketData::List(vec![
                            PacketData::Integer(5),
                            PacketData::Integer(6),
                            PacketData::Integer(7),
                        ]),
                    ]),
                ]),
            ]),
            PacketData::Integer(8),
            PacketData::Integer(9),
        ]);
        assert_eq!(output, expected);
    }
}
