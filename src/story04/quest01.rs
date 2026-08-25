use crate::util::parse::*;
use std::collections::HashSet;

pub fn part1(notes: &str) -> i32 {
    recaman(notes, true)
}

pub fn part2(notes: &str) -> i32 {
    recaman(notes, false)
}

pub fn part3(notes: &str) -> i32 {
    notes
        .lines()
        .map(|line| {
            let mut parity = 0;
            let mut arcs = [Vec::new(), Vec::new()];

            line.iter_signed::<i32>().fold(0, |position, jump| {
                let side = &mut arcs[parity];
                let contains = |(start, end), point| start <= point && point <= end;
                let crossing =
                    |next| side.iter().any(|&arc| contains(arc, next) && !contains(arc, position));

                let (lower, upper) = side
                    .iter()
                    .filter(|&&arc| contains(arc, position))
                    .fold((0, i32::MAX), |(lo, hi), &(start, end)| (lo.max(start), hi.min(end)));

                let (backwards, forwards) = (position - jump, position + jump);
                let next = (backwards > lower && !crossing(backwards))
                    .then_some(backwards)
                    .or_else(|| (forwards..upper).find(|&next| !crossing(next)));

                if let Some(next) = next {
                    side.push((position.min(next), position.max(next)));
                    parity ^= 1;
                    next
                } else {
                    position
                }
            })
        })
        .sum()
}

fn recaman(notes: &str, allow_visited: bool) -> i32 {
    notes
        .lines()
        .map(|line| {
            let mut seen = HashSet::new();

            line.iter_signed::<i32>().fold(0, |position, jump| {
                let (backwards, forwards) = (position - jump, position + jump);

                if backwards > 0 && seen.insert(backwards) {
                    backwards
                } else {
                    (forwards..).find(|&next| seen.insert(next) || allow_visited).unwrap()
                }
            })
        })
        .sum()
}
