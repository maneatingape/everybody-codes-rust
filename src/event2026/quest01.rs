use crate::util::parse::*;

pub fn part1(notes: &str) -> u32 {
    solve(notes, |cones| {
        let side = (cones - 1).isqrt() + 1;
        side * side - cones
    })
}

pub fn part2(notes: &str) -> u32 {
    solve(notes, |cones| {
        let side = cones.div_ceil(4).max(2);
        4 * side - cones
    })
}

pub fn part3(notes: &str) -> u32 {
    solve(notes, |cones| {
        let side = (cones.isqrt() + 1) | 1;
        side * side - cones - 1
    })
}

fn solve(notes: &str, rule: fn(u32) -> u32) -> u32 {
    notes.iter_unsigned().map(rule).sum()
}
