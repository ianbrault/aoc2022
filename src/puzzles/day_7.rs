/*
** src/puzzles/day_7.rs
** https://adventofcode.com/2022/day/7
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;
use log::debug;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::mem;
use std::path::PathBuf;

const CD_LEN: usize = 5;
const DIR_LEN: usize = 4;

#[derive(Clone, Debug)]
struct DirListing<'a> {
    path: PathBuf,
    file_sizes: u64,
    subdirs: Vec<&'a str>,
}

impl<'a> DirListing<'a> {
    fn new(path: PathBuf, file_sizes: u64, subdirs: Vec<&'a str>) -> Self {
        Self {
            path,
            file_sizes,
            subdirs,
        }
    }

    fn is_leaf_node(&self) -> bool {
        self.subdirs.is_empty()
    }
}

/// provides a double-buffered approach to pull items out of one buffer (the
/// sink) and push items into the other (the drain) at the same time
struct SinkDrainBuffer<T> {
    sink: Vec<T>,
    drain: Vec<T>,
}

impl<T> SinkDrainBuffer<T> {
    fn is_empty(&self) -> bool {
        self.sink.is_empty() && self.drain.is_empty()
    }

    fn pop(&mut self) -> Option<T> {
        self.sink.pop()
    }

    fn push(&mut self, t: T) {
        self.drain.push(t);
    }

    fn swap(&mut self) {
        mem::swap(&mut self.sink, &mut self.drain);
    }
}

impl<T> FromIterator<T> for SinkDrainBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let sink = iter.into_iter().collect();
        let drain = Vec::new();
        Self { sink, drain }
    }
}

fn path_from_stack(dir_stack: &[&str]) -> PathBuf {
    PathBuf::from("/").join(&dir_stack[1..dir_stack.len()].join("/"))
}

fn parse_dir_listings(input: &str) -> Vec<DirListing<'_>> {
    let lines = utils::split_lines(input).collect::<Vec<_>>();
    let nlines = lines.len();

    let mut listings = Vec::new();
    let mut dir_stack = Vec::new();

    // iterate over each line and group into directory listings
    let mut i = 0;
    while i < nlines {
        let line = &lines[i];
        // the first line in each directory listing is a cd into the directory
        // grab the directory name
        let name = &line[CD_LEN..line.len()];
        if name == ".." {
            // if this is a cd into the parent directory, pop the new current
            // working off the directory stack and continue
            let _ = dir_stack.pop().unwrap();
            debug!(
                "line {:03}: changing to parent directory {:?}",
                i,
                path_from_stack(&dir_stack)
            );
            i += 1;
        } else {
            // otherwise set it as the current working directory and add it to
            // the directory stack
            dir_stack.push(name);
            let path = path_from_stack(&dir_stack);
            debug!("line {:03}: changing to directory {:?}", i, path);
            // the next line will be an ls command
            i += 2;
            // parse the directory entries until the next cd is reached
            let mut file_sizes = 0;
            let mut subdirs = Vec::new();
            while i < nlines && !lines[i].starts_with('$') {
                let line = &lines[i];
                if line.starts_with("dir") {
                    // this is a subdirectory entry
                    // grab the name and add it to the list
                    let subdir = &line[DIR_LEN..line.len()];
                    debug!(
                        "line {:03}: directory {:?} has sub-directory {}",
                        i, path, subdir
                    );
                    subdirs.push(subdir);
                    i += 1;
                } else {
                    // otherwise this is a file entry
                    // grab the file size and add it to the sum
                    let sep = line.find(' ').unwrap();
                    let size = line[..sep].parse::<u64>().unwrap();
                    let file = &line[(sep + 1)..line.len()];
                    debug!(
                        "line {:03}: directory {:?} has file {} with size {}",
                        i, path, file, size
                    );
                    file_sizes += size;
                    i += 1;
                }
            }
            // finally, create the directory listing object and add to the list
            let listing = DirListing::new(path, file_sizes, subdirs);
            debug!("adding new listing {:?}", listing);
            listings.push(listing);
        }
    }

    listings
}

fn calculate_dir_sizes<'a>(listings: &'a [DirListing<'a>]) -> HashMap<&'a PathBuf, u64> {
    let mut sizes = HashMap::new();
    let mut buffer = SinkDrainBuffer::from_iter(listings.iter());

    // initial pass, add leaf nodes
    while let Some(listing) = buffer.pop() {
        if listing.is_leaf_node() {
            sizes.insert(&listing.path, listing.file_sizes);
        } else {
            buffer.push(listing);
        }
    }
    buffer.swap();

    // complete subsequent passes, adding paths with known child nodes
    while !buffer.is_empty() {
        // on each pass, find listings for whom all subdirectories already have
        // known sizes
        while let Some(listing) = buffer.pop() {
            let subdir_paths = listing
                .subdirs
                .iter()
                .map(|path| listing.path.join(path))
                .collect::<Vec<_>>();
            if subdir_paths.iter().all(|path| sizes.contains_key(&path)) {
                let subdir_sizes = subdir_paths
                    .iter()
                    .map(|path| sizes.get(path).unwrap())
                    .sum::<u64>();
                sizes.insert(&listing.path, listing.file_sizes + subdir_sizes);
            } else {
                buffer.push(listing);
            }
        }
        buffer.swap();
    }

    sizes
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the directory listings out of the input
    let listings = parse_dir_listings(&input);
    // and calculate the size of each directory in the tree
    let dir_sizes = calculate_dir_sizes(&listings);

    // part 1: Find all of the directories with a total size of at most 100000.
    // What is the sum of the total sizes of those directories?
    let max_size = 100000;
    let dir_size_sum = dir_sizes
        .iter()
        .filter(|(_, &size)| size <= max_size)
        .map(|(_, &size)| size)
        .sum::<u64>();
    solution.set_part_1(dir_size_sum);

    // part 2: Find the smallest directory that, if deleted, would free up
    // enough space on the filesystem to run the update. What is the total size
    // of that directory?
    let space_available = 70000000;
    let update_space = 30000000;
    let max_space_for_update = space_available - update_space;
    let total_size = *dir_sizes.get(&PathBuf::from("/")).unwrap() as i64;
    let space_to_delete = total_size - max_space_for_update;
    // we need a directory that is larger than the space needed to delete but
    // to minimize this gap, use the difference as the sort key and find the
    // smallest negative number
    let (dir_to_delete, _) = dir_sizes
        .iter()
        .map(|(path, &size)| (path, space_to_delete - (size as i64)))
        .filter(|(_, size)| *size <= 0)
        .max_by_key(|(_, size)| *size)
        .unwrap();
    let deleted_dir_size = *dir_sizes.get(dir_to_delete).unwrap();
    solution.set_part_2(deleted_dir_size);

    Ok(solution)
}
