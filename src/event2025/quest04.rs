use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    let (first, last, _) = parse(notes);
    (2025 * first) / last
}

pub fn part2(notes: &str) -> u64 {
    let (first, last, _) = parse(notes);
    (10_000_000_000_000 * last).div_ceil(first)
}

pub fn part3(notes: &str) -> u64 {
    let (first, last, gears) = parse(notes);
    gears[1..].chunks_exact(2).fold(100 * first, |turns, chunk| turns * chunk[1] / chunk[0]) / last
}

fn parse(notes: &str) -> (u64, u64, Vec<u64>) {
    let gears: Vec<_> = notes.iter_unsigned().collect();
    let first = *gears.first().unwrap();
    let last = *gears.last().unwrap();
    (first, last, gears)
}
