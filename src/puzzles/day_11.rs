/*
** src/puzzles/day_11.rs
** https://adventofcode.com/2022/day/11
*/

use crate::types::Solution;
use crate::utils;

use anyhow::Result;

#[cfg(feature = "sample")]
const N_MONKEYS: usize = 4;
#[cfg(not(feature = "sample"))]
const N_MONKEYS: usize = 8;

const N_ROUNDS_1: usize = 20;
const N_ROUNDS_2: usize = 10000;

// constants used for parsing monkey specifications
const LINES_PER_MONKEY: usize = 6;
const STARTING_ITEMS_PFIX: usize = 18;
const OPERATION_PFIX: usize = 23;
const TEST_PFIX: usize = 21;
const MONKEY_IF_TRUE_PFIX: usize = 29;
const MONKEY_IF_FALSE_PFIX: usize = 30;

type Operation = Box<dyn Fn(u64) -> u64>;

struct Item {
    monkey: usize,
    item: u64,
}

impl Item {
    fn new(monkey: usize, item: u64) -> Self {
        Self { monkey, item }
    }
}

fn parse_items(lines: &[&str]) -> Vec<Item> {
    let mut items = Vec::new();
    for (monkey, item_line) in lines.iter().skip(1).step_by(LINES_PER_MONKEY).enumerate() {
        for item in item_line[STARTING_ITEMS_PFIX..].split(", ") {
            items.push(Item::new(monkey, item.parse().unwrap()));
        }
    }
    items
}

fn parse_operation(s: &str) -> Operation {
    let op = s.chars().next().unwrap();
    let value = &s[2..];
    match op {
        '+' => {
            let x = value.parse::<u64>().unwrap();
            Box::new(move |n| n + x)
        }
        '*' => match value {
            "old" => Box::new(|n| n * n),
            _ => {
                let x = value.parse::<u64>().unwrap();
                Box::new(move |n| n * x)
            }
        },
        _ => unreachable!(),
    }
}

fn parse_operations(lines: &[&str]) -> Vec<Operation> {
    lines
        .iter()
        .skip(2)
        .step_by(LINES_PER_MONKEY)
        .map(|s| parse_operation(&s[OPERATION_PFIX..]))
        .collect()
}

fn parse_divisors(lines: &[&str]) -> Vec<u64> {
    lines
        .iter()
        .skip(3)
        .step_by(LINES_PER_MONKEY)
        .map(|s| s[TEST_PFIX..].parse().unwrap())
        .collect()
}

fn parse_next_monkeys(lines: &[&str]) -> Vec<(usize, usize)> {
    let monkeys_if_true = lines
        .iter()
        .skip(4)
        .step_by(LINES_PER_MONKEY)
        .map(|s| s[MONKEY_IF_TRUE_PFIX..].parse().unwrap());
    let monkeys_if_false = lines
        .iter()
        .skip(5)
        .step_by(LINES_PER_MONKEY)
        .map(|s| s[MONKEY_IF_FALSE_PFIX..].parse().unwrap());
    monkeys_if_true.zip(monkeys_if_false).collect()
}

fn do_round(
    items: &mut [Item],
    operation: &Operation,
    divisor: u64,
    next_monkey: (usize, usize),
    monkey: usize,
    inspections: &mut u64,
) {
    let (if_true, if_false) = next_monkey;
    // only consider items for the current monkey
    for item in items.iter_mut().filter(|i| i.monkey == monkey) {
        *inspections += 1;
        // the monkey modifies the worry level according to its operation
        item.item = operation(item.item);
        // worry level is divided by 3 as the monkey gets bored
        item.item /= 3;
        // now apply the divisibility test and throw to another monkey
        item.monkey = if item.item % divisor == 0 {
            if_true
        } else {
            if_false
        };
    }
}

fn do_rounds(
    items: &mut [Item],
    operations: &[Operation],
    divisors: &[u64],
    next_monkeys: &[(usize, usize)],
    n_rounds: usize,
) -> u64 {
    let mut inspections = vec![0; N_MONKEYS];

    // run all rounds, for each monkey
    for _ in 0..n_rounds {
        for monkey in 0..N_MONKEYS {
            do_round(
                items,
                &operations[monkey],
                divisors[monkey],
                next_monkeys[monkey],
                monkey,
                &mut inspections[monkey],
            );
        }
    }

    // calculate and return the monkey business
    inspections.sort();
    inspections[N_MONKEYS - 1] * inspections[N_MONKEYS - 2]
}

fn do_round_extra_worry(
    items: &mut [Item],
    operation: &Operation,
    divisor: u64,
    next_monkey: (usize, usize),
    reduction: u64,
    monkey: usize,
    inspections: &mut u64,
) {
    let (if_true, if_false) = next_monkey;
    // only consider items for the current monkey
    for item in items.iter_mut().filter(|i| i.monkey == monkey) {
        *inspections += 1;
        // the monkey modifies the worry level according to its operation
        item.item = operation(item.item);
        // we can apply the reduction here, see below for details
        item.item %= reduction;
        // now apply the divisibility test and throw to another monkey
        item.monkey = if item.item % divisor == 0 {
            if_true
        } else {
            if_false
        };
    }
}

fn do_rounds_extra_worry(
    items: &mut [Item],
    operations: &[Operation],
    divisors: &[u64],
    next_monkeys: &[(usize, usize)],
    reduction: u64,
    n_rounds: usize,
) -> u64 {
    let mut inspections = vec![0; N_MONKEYS];

    // run all rounds, for each monkey
    for _ in 0..n_rounds {
        for monkey in 0..N_MONKEYS {
            do_round_extra_worry(
                items,
                &operations[monkey],
                divisors[monkey],
                next_monkeys[monkey],
                reduction,
                monkey,
                &mut inspections[monkey],
            );
        }
    }

    // calculate and return the monkey business
    inspections.sort();
    inspections[N_MONKEYS - 1] * inspections[N_MONKEYS - 2]
}

pub fn run(input: String) -> Result<Solution> {
    let mut solution = Solution::new();
    // parse the monkeys
    let lines = utils::split_lines(&input)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let mut items_a = parse_items(&lines);
    let mut items_b = parse_items(&lines);
    let operations = parse_operations(&lines);
    let divisors = parse_divisors(&lines);
    let next_monkeys = parse_next_monkeys(&lines);

    // part 1: Figure out which monkeys to chase by counting how many items
    // they inspect over 20 rounds. What is the level of monkey business after
    // 20 rounds of stuff-slinging simian shenanigans?
    let monkey_business = do_rounds(
        &mut items_a,
        &operations,
        &divisors,
        &next_monkeys,
        N_ROUNDS_1,
    );
    solution.set_part_1(monkey_business);

    // part 2: Worry levels are no longer divided by three after each item is
    // inspected; you'll need to find another way to keep your worry levels
    // manageable. Starting again from the initial state in your puzzle input,
    // what is the level of monkey business after 10000 rounds?
    // had to do quite a bit of Googling to figure this out...
    // to keep the worry levels manageable, the items can be reduced by taking
    // the modulo of the product of all divisbility tests; observe that these
    // are all prime numbers, then we can use the fact that, if A and B are
    // prime numbers, N % A == (N % (A*B)) % A and N % B == (N % (A*B)) % B
    let reduction = divisors.iter().product();
    let monkey_business = do_rounds_extra_worry(
        &mut items_b,
        &operations,
        &divisors,
        &next_monkeys,
        reduction,
        N_ROUNDS_2,
    );
    solution.set_part_2(monkey_business);

    Ok(solution)
}
