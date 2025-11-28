use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> u32 {
    solve(notes)
}

pub fn part2(notes: &str) -> u32 {
    solve(notes)
}

pub fn part3(notes: &str) -> u32 {
    solve(notes)
}

fn solve(notes: &str) -> u32 {
    let mut walls: Vec<_> = notes.iter_unsigned::<u32>().chunk::<3>().collect();
    walls.dedup_by_key(|[x1, ..]| *x1);
    walls.into_iter().map(|[x1, y1, _]| (x1 + y1).div_ceil(2)).max().unwrap()
}
