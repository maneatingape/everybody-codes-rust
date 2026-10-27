use std::collections::HashSet;

use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    solve(notes)
}

pub fn part2(notes: &str) -> usize {
    solve(notes)
}

pub fn part3(notes: &str) -> usize {
    solve(notes)
}

fn solve(notes: &str) -> usize {
    notes.lines().map(order).sum()
}

fn order(alloy: &str) -> usize {
    let pairs = alloy.iter_unsigned::<usize>().chunk::<2>();
    let order = alloy.bytes().skip(1).step_by(4);

    let mut digits = HashSet::new();
    let mut harder = [[true; 10]; 10];
    let mut number = 0;

    for ([first, second], op) in pairs.zip(order) {
        let (from, to) = if op == b'<' { (first, second) } else { (second, first) };
        digits.extend([from, to]);
        harder[from][to] = false;
    }

    while let Some(&digit) = digits.iter().find(|&&from| digits.iter().all(|&to| harder[from][to]))
    {
        digits.remove(&digit);
        number = 10 * number + digit;
    }

    number
}
