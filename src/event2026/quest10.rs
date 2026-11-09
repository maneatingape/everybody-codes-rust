use std::collections::HashMap;

use crate::util::grid::*;
use crate::util::point::*;

type Ghost = (Point, Point, i64);

pub fn part1(notes: &str) -> i64 {
    let (grid, mut ghosts) = parse(notes, i64::MAX);
    (0..10).for_each(|_| step(&grid, &mut ghosts));
    ghosts.iter().map(|&(.., lives)| i64::MAX - lives).sum()
}

pub fn part2(notes: &str) -> i64 {
    solve(notes, 10_000)
}

pub fn part3(notes: &str) -> i64 {
    solve(notes, 1_000_000_000_000)
}

fn parse(notes: &str, lives: i64) -> (Grid<u8>, Vec<Ghost>) {
    let grid = Grid::parse(notes);
    let ghosts = grid
        .points()
        .filter(|&point| !matches!(grid[point], b'#' | b'.'))
        .map(|point| (point, Point::from(grid[point]), lives))
        .collect();
    (grid, ghosts)
}

pub fn solve(notes: &str, lives: i64) -> i64 {
    let (grid, mut ghosts) = parse(notes, lives);
    let mut rounds = 0;
    let mut seen = HashMap::new();

    while !ghosts.is_empty() {
        step(&grid, &mut ghosts);
        rounds += 1;

        let key: Vec<_> =
            ghosts.iter().map(|&(position, direction, _)| (position, direction)).collect();

        if let Some((start, previous)) = seen.insert(key, (rounds, ghosts.clone())) {
            let skip = ghosts
                .iter()
                .zip(&previous)
                .map(|(&(.., now), &(.., then))| (now - 1) / (then - now))
                .min()
                .unwrap();

            for ((.., now), (.., then)) in ghosts.iter_mut().zip(previous) {
                *now -= skip * (then - *now);
            }

            rounds += skip * (rounds - start);
        }
    }

    rounds
}

fn step(grid: &Grid<u8>, ghosts: &mut Vec<Ghost>) {
    let mut same = HashMap::new();

    for (position, direction, lives) in ghosts.iter_mut() {
        let next = *position + *direction;

        if grid[next] == b'#' {
            *direction = direction.clockwise();
            *lives -= 1;
        } else {
            *position = next;
        }

        *same.entry(*position).or_insert(0) += u8::from(*lives > 0);
    }

    ghosts.retain_mut(|(position, direction, lives)| {
        if same[position] > 1 {
            *direction = *direction * -1;
            *lives -= 1;
        }
        *lives > 0
    });
}
