use std::cmp::Reverse;

use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    solve(notes.iter_unsigned().chunk::<2>().map(|[x, y]| [x, y, 1]).collect())
}

pub fn part2(notes: &str) -> u64 {
    solve(notes.iter_unsigned().chunk::<3>().collect())
}

pub fn part3(notes: &str) -> u64 {
    solve(notes.iter_unsigned().chunk::<3>().collect())
}

fn solve(mut stacks: Vec<[u64; 3]>) -> u64 {
    stacks.sort_by_key(|&[_, y, _]| Reverse(y));
    let x = stacks[0][0];

    let (units, ..) = stacks.iter().fold((0, x, x), |(units, left, right), &[x, y, w]| {
        let extra = left - left.min(x) + right.max(x + w) - right;
        (units + y * extra, left.min(x), right.max(x + w))
    });

    units
}
