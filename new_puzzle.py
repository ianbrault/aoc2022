#!/usr/bin/env python3

import os
import pathlib
import sys

mod_template = """\
/*
** src/puzzles/mod.rs
*/

<M>

use crate::types::Puzzle;

pub const N_DAYS: usize = <N>;

pub const DAYS: [Puzzle; N_DAYS] = [
<P>
];
"""

puzzle_template = """\
/*
** src/puzzles/day_<D>.rs
** https://adventofcode.com/2022/day/<D>
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();

    // part 1: ???

    // part 2: ???

    Ok(solution)
}

"""


if __name__ == "__main__":
    try:
        n = int(sys.argv[1])
    except IndexError:
        sys.exit("error: missing argument DAY")
    except ValueError:
        sys.exit(f"error: invalid argument DAY")
    add_sample = "-s" in sys.argv or "--sample" in sys.argv

    current_dir = os.path.dirname(os.path.abspath(__file__))
    puzzle_dir = os.path.join(current_dir, "src", "puzzles")
    input_dir = os.path.join(current_dir, "input")

    # write the puzzle source file
    with open(os.path.join(puzzle_dir, f"day_{n}.rs"), "w") as puzzle_file:
        puzzle_file.write(puzzle_template.replace("<D>", str(n)))

    # write the mod.rs file
    with open(os.path.join(puzzle_dir, "mod.rs"), "w") as mod_file:
        # sort to match rustfmt
        mods = "\n".join(sorted(f"mod day_{i + 1};" for i in range(n)))
        puzzles = "\n".join(f"    day_{i + 1}::run," for i in range(n))
        mod_file.write(
            mod_template
                .replace("<N>", str(n))
                .replace("<M>", mods)
                .replace("<P>", puzzles))

    # touch the input file
    pathlib.Path(os.path.join(input_dir, f"D{n}.txt")).touch()
    # touch the sample input file, if requested
    if add_sample:
        pathlib.Path(os.path.join(input_dir, f"D{n}.dbg.txt")).touch()
