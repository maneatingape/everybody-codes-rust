use std::collections::{BTreeMap, HashMap};

use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    solve(notes, elements)
}

pub fn part2(notes: &str) -> u64 {
    solve(notes, leaves)
}

pub fn part3(notes: &str) -> u64 {
    solve(notes, leaves)
}

fn solve(notes: &str, simulate: fn(u64, u64) -> u64) -> u64 {
    notes.iter_unsigned().chunk::<3>().map(|[_, growth, limit]| simulate(growth, limit)).sum()
}

fn elements(growth: u64, limit: u64) -> u64 {
    let mut todo = HashMap::from([(growth, 1)]);
    let mut elements = 1;
    let mut branches = 1;
    let mut time = 1;

    while elements < limit {
        time += 1;
        let count = todo.remove(&time).unwrap_or(0);

        *todo.entry(time + growth / 2 + 1).or_insert(0) += 2 * count;
        *todo.entry(time + growth - 1).or_insert(0) += count;

        elements += branches + count;
        branches += 2 * count;
    }

    time
}

fn leaves(growth: u64, limit: u64) -> u64 {
    let mut todo = BTreeMap::from([(growth, 1)]);
    let mut leaves = 1;
    let mut time = 1;

    while leaves < limit {
        let (next, count) = todo.pop_first().unwrap();

        *todo.entry(next + growth / 2 + 1).or_insert(0) += 2 * count;
        *todo.entry(next + growth - 1).or_insert(0) += count;

        leaves += count;
        time = next;
    }

    time
}
