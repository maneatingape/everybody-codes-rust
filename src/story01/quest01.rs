use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;
use std::collections::HashMap;

pub fn part1(notes: &str) -> usize {
    solve(notes, |n, e, m| eni(1, n, e, m))
}

pub fn part2(notes: &str) -> usize {
    solve(notes, |n, e, m| eni(n.mod_pow(e - 5, m), n, 5, m))
}

pub fn part3(notes: &str) -> usize {
    solve(notes, find_cycle)
}

pub fn solve(notes: &str, wrapper: fn(usize, usize, usize) -> usize) -> usize {
    notes
        .iter_unsigned::<usize>()
        .chunk::<7>()
        .map(|[a, b, c, x, y, z, m]| wrapper(a, x, m) + wrapper(b, y, m) + wrapper(c, z, m))
        .max()
        .unwrap()
}

fn eni(mut score: usize, n: usize, e: usize, m: usize) -> usize {
    let mut result = 0;
    let mut power = 0;

    for _ in 0..e {
        score = (score * n) % m;
        result += score * 10_usize.pow(power);
        power += 1 + if score < 10 { 0 } else { score.ilog10() };
    }

    result
}

fn find_cycle(n: usize, e: usize, m: usize) -> usize {
    let mut score = 1;
    let mut total = 0;
    let mut index = 0;

    let mut sums = Vec::with_capacity(m);
    let mut seen = HashMap::with_capacity(m);

    sums.push(0);

    loop {
        score = (score * n) % m;
        total += score;
        index += 1;

        if let Some(previous) = seen.insert(score, index) {
            let cycle_length = index - previous;
            let cycle_total = total - sums[previous];

            let quotient = (e - index + 1) / cycle_length;
            let remainder = (e - index + 1) % cycle_length;

            return (total - score)
                + (quotient * cycle_total)
                + (sums[previous + remainder - 1] - sums[previous - 1]);
        }

        sums.push(total);
    }
}
