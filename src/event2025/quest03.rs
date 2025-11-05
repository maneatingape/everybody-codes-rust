use crate::util::parse::*;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn part1(notes: &str) -> u32 {
    let unique: HashSet<u32> = notes.iter_unsigned().collect();
    unique.iter().sum()
}

pub fn part2(notes: &str) -> u32 {
    let sorted: BTreeSet<u32> = notes.iter_unsigned().collect();
    sorted.iter().take(20).sum()
}

pub fn part3(notes: &str) -> u32 {
    let mut frequency = HashMap::new();
    notes.iter_unsigned::<u32>().for_each(|n| *frequency.entry(n).or_insert(0) += 1);
    frequency.into_values().max().unwrap()
}
