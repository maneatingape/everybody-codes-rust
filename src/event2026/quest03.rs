use std::iter::successors;

use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    notes.iter_unsigned().chunk::<2>().map(|[base, vote]| parse(base, vote)).sum()
}

pub fn part2(notes: &str) -> u64 {
    notes.iter_unsigned().chunk::<2>().map(|[base, vote]| parse(base, vote)).sum()
}

pub fn part3(notes: &str) -> u64 {
    notes
        .iter_unsigned()
        .chunk::<3>()
        .filter_map(|[_, base, vote]| {
            let number = parse(base, vote);
            (0..vote).all(|smaller| parse(base, smaller) != number).then_some(number)
        })
        .sum()
}

fn parse(base: u64, vote: u64) -> u64 {
    digits(vote)
        .zip(digits(base).cycle())
        .zip(0..)
        .map(|((digit, radix), exp)| digit * radix.pow(exp))
        .sum()
}

fn digits(n: u64) -> impl Iterator<Item = u64> + Clone {
    successors(Some(n), |&n| (n >= 10).then_some(n / 10)).map(|n| n % 10)
}
