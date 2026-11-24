use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    solve(notes, |n, e, m| eni(1, n, e, m))
}

pub fn part2(notes: &str) -> usize {
    solve(notes, |n, e, m| eni(n.mod_pow(e - 5, m), n, 5, m))
}

pub fn part3(notes: &str) -> usize {
    solve(notes, find_cycle)
}

fn solve(notes: &str, wrapper: fn(usize, usize, usize) -> usize) -> usize {
    notes
        .iter_unsigned()
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
        power += if score < 10 { 1 } else { score.ilog10() + 1 };
    }

    result
}

fn find_cycle(n: usize, e: usize, m: usize) -> usize {
    let mut score = 1;
    let mut sums = Vec::with_capacity(m + 1);
    let mut seen = vec![0; m];

    sums.push(0);

    loop {
        score = (score * n) % m;
        let index = sums.len();
        let total = sums[index - 1];
        let previous = seen[score];

        if previous > 0 {
            let start = previous - 1;
            let cycle_length = index - previous;
            let cycle_total = total - sums[start];
            let steps = e - start;

            return sums[start + steps % cycle_length] + (steps / cycle_length) * cycle_total;
        }

        seen[score] = index;
        sums.push(total + score);
    }
}
