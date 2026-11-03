use std::collections::HashMap;

use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    simulate(notes, None, 5, usize::MAX)
}

pub fn part2(notes: &str) -> usize {
    simulate(notes, None, 10, usize::MAX)
}

pub fn part3(notes: &str) -> usize {
    part3_testable(notes, 1_000_000)
}

pub fn part3_testable(notes: &str, limit: usize) -> usize {
    (0..notes.lines().count()).map(|start| simulate(notes, Some(start), 10, limit)).max().unwrap()
}

fn simulate(notes: &str, start: Option<usize>, strength: usize, limit: usize) -> usize {
    let (mut energy, connections): (Vec<_>, Vec<Vec<usize>>) = notes
        .lines()
        .map(|line| {
            let mut iter = line.iter_unsigned().skip(1);
            let energy = iter.next();
            let connections = iter.map(|id| id - 1).collect();
            (energy, connections)
        })
        .collect();

    let start = start.unwrap_or_else(|| {
        (0..connections.len()).max_by_key(|&index| connections[index].len()).unwrap()
    });
    let mut current = HashMap::from([((start, strength), 1)]);
    let mut next = HashMap::new();

    while !current.is_empty() {
        for ((index, strength), count) in current.drain() {
            if let Some(energy) = &mut energy[index] {
                *energy += strength * count;

                if strength > 1 {
                    for &child in &connections[index] {
                        *next.entry((child, strength - 1)).or_insert(0) += count;
                    }
                }
            }
        }

        for (children, energy) in connections.iter().zip(&mut energy) {
            if energy.take_if(|energy| *energy >= limit).is_some() {
                for &child in children {
                    *next.entry((child, 10)).or_insert(0) += 1;
                }
            }
        }

        (current, next) = (next, current);
    }

    energy.iter().flatten().sum()
}
