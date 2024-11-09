use crate::util::parse::*;

pub fn part1(notes: &str) -> u32 {
    strikes(notes, |xs| xs[0])
}

pub fn part2(notes: &str) -> u32 {
    strikes(notes, |xs| xs[0])
}

pub fn part3(notes: &str) -> u32 {
    strikes(notes, |xs| xs[xs.len() / 2])
}

fn strikes(notes: &str, f: fn(&[u32]) -> u32) -> u32 {
    let mut xs: Vec<_> = notes.iter_unsigned().collect();
    xs.sort_unstable();

    let target = f(&xs);
    xs.iter().map(|x| x.abs_diff(target)).sum()
}
