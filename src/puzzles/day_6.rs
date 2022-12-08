/*
** src/puzzles/day_6.rs
** https://adventofcode.com/2022/day/6
*/

use crate::types::{Error, Solution};

use anyhow::Result;

const N_CHARS: usize = 26;
const CHAR_BASE: u32 = 'a' as u32;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

const fn char_index(c: char) -> usize {
    ((c as u32) - CHAR_BASE) as usize
}

/// tracks whether all characters in a stream are unique
struct UniqueCharCounter {
    counts: [u32; N_CHARS],
}

impl UniqueCharCounter {
    fn new() -> Self {
        let counts = [0; N_CHARS];
        Self { counts }
    }

    fn add(&mut self, c: char) {
        let i = char_index(c);
        self.counts[i] += 1;
    }

    fn remove(&mut self, c: char) {
        let i = char_index(c);
        self.counts[i] -= 1;
    }

    fn all_unique(&self) -> bool {
        // if all counts are 0 or 1, no bits higher than the lowest will be set
        // once all counts are or-ed together
        let mash = self
            .counts
            .into_iter()
            .reduce(|acc, cnt| acc | cnt)
            .unwrap();
        (mash & (!0x1)) == 0
    }
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // split input into an array of characters
    let stream = input.chars().collect::<Vec<_>>();
    let size = stream.len();
    // initialize counters for start-of-packet and start-of-message searches
    let mut packet_char_counter = UniqueCharCounter::new();
    let mut message_char_counter = UniqueCharCounter::new();

    // part 1: How many characters need to be processed before the first
    // start-of-packet marker is detected?

    // initialize with the first characters
    for c in &stream[..PACKET_MARKER_SIZE] {
        packet_char_counter.add(*c);
    }
    // then use a sliding window to find the start-of-packet marker
    let mut wi = 0;
    let mut wj = PACKET_MARKER_SIZE;
    while wj < size && !packet_char_counter.all_unique() {
        // add the next character to the window and remove the character from
        // the start of the old window
        packet_char_counter.remove(stream[wi]);
        packet_char_counter.add(stream[wj]);
        wi += 1;
        wj += 1;
    }

    let start_of_packet = if wj == size {
        Err(Error::NoSolution)
    } else {
        Ok(wj)
    };
    solution.set_part_1(start_of_packet?);

    // part 2: How many characters need to be processed before the first
    // start-of-message marker is detected?

    // initialize with the first characters
    for c in &stream[..MESSAGE_MARKER_SIZE] {
        message_char_counter.add(*c);
    }
    // then use a sliding window to find the start-of-packet marker
    let mut wi = 0;
    let mut wj = MESSAGE_MARKER_SIZE;
    while wj < size && !message_char_counter.all_unique() {
        // add the next character to the window and remove the character from
        // the start of the old window
        message_char_counter.remove(stream[wi]);
        message_char_counter.add(stream[wj]);
        wi += 1;
        wj += 1;
    }

    let start_of_message = if wj == size {
        Err(Error::NoSolution)
    } else {
        Ok(wj)
    };
    solution.set_part_2(start_of_message?);

    Ok(solution)
}
