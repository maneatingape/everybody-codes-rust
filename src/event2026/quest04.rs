use std::cmp::Reverse;

use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    solve(notes)
}

pub fn part2(notes: &str) -> usize {
    solve(notes)
}

pub fn part3(notes: &str) -> usize {
    let mut rows: Vec<_> = notes.iter_unsigned::<usize>().chunk::<4>().collect();
    rows.sort_unstable();

    for group in rows.chunk_by_mut(|first, second| first[..2] == second[..2]) {
        group.iter_mut().fold(0, |sum, [.., time]| {
            *time += sum;
            *time
        });
    }

    let mut scores = vec![0; rows.len()];
    rows.sort_unstable_by_key(|&[_, rest @ ..]| rest);

    for group in rows.chunk_by(|first, second| first[1..3] == second[1..3]) {
        for &[id, _, part, time] in group {
            let position = group.partition_point(|&[.., other]| other < time);
            scores[id] += (50 * part).saturating_sub(position);
        }
    }

    scores.sort_unstable_by_key(|&score| Reverse(score));
    scores.iter().map(|&score| score * (1 + scores.partition_point(|&other| other > score))).sum()
}

fn solve(notes: &str) -> usize {
    let rows: Vec<_> = notes.iter_unsigned::<usize>().chunk::<4>().collect();
    let mut scores = vec![(0, 0); rows.len()];

    for [id, .., time] in rows {
        scores[id].0 -= 1;
        scores[id].1 += time;
    }

    scores.sort_unstable();
    scores.iter().zip(1..).map(|((_, time), rank)| time * rank).sum()
}
