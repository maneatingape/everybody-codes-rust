use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;
use std::collections::HashSet;

pub fn part1(notes: &str) -> i32 {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);
    suffix.lines().enumerate().map(|(slot, behavior)| drop(&grid, behavior, slot as i32)).sum()
}

pub fn part2(notes: &str) -> i32 {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);
    suffix
        .lines()
        .map(|behavior| {
            (0..grid.width).step_by(2).map(|start| drop(&grid, behavior, start / 2)).max().unwrap()
        })
        .sum()
}

pub fn part3(notes: &str) -> String {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);

    let tokens: Vec<Vec<_>> = suffix
        .lines()
        .map(|behavior| {
            let mut row: Vec<_> = (0..grid.width)
                .step_by(2)
                .map(|start| (start / 2, drop(&grid, behavior, start / 2)))
                .collect();
            row.sort_unstable_by_key(|token| token.1);
            row
        })
        .collect();

    format!("{} {}", min_dijkstra(&tokens), max_dijkstra(&tokens))
}

fn drop(grid: &Grid<u8>, behavior: &str, slot: i32) -> i32 {
    let mut x = slot * 2;
    let mut iter = behavior.bytes();

    for y in 0..grid.height {
        if grid[Point::new(x, y)] == b'*' {
            let direction = iter.next().unwrap();
            x += if direction == b'R' { 1 } else { -1 };

            if x < 0 {
                x += 2;
            }
            if x >= grid.width {
                x -= 2;
            }
        }
    }

    (x - slot + 1).max(0)
}

fn min_dijkstra(tokens: &[Vec<(i32, i32)>]) -> i32 {
    let start = 0;
    let coins = tokens.iter().map(|token| token[start].1).sum::<i32>();
    let state = [start; 6];

    let mut todo = MinHeap::from([(coins, state)]);
    let mut seen = HashSet::from([state]);

    while let Some((coins, state)) = todo.pop() {
        if unique(tokens, &state) {
            return coins;
        }

        for i in 0..6 {
            let mut next_state = state;
            next_state[i] += 1;

            let next_coins = coins - tokens[i][state[i]].1 + tokens[i][next_state[i]].1;

            if seen.insert(next_state) {
                todo.push(next_coins, next_state);
            }
        }
    }

    unreachable!()
}

fn max_dijkstra(tokens: &[Vec<(i32, i32)>]) -> i32 {
    let start = tokens[0].len() - 1;
    let coins = -tokens.iter().map(|token| token[start].1).sum::<i32>();
    let state = [start; 6];

    let mut todo = MinHeap::from([(coins, state)]);
    let mut seen = HashSet::from([state]);

    while let Some((coins, state)) = todo.pop() {
        if unique(tokens, &state) {
            return -coins;
        }

        for i in 0..6 {
            let mut next_state = state;
            next_state[i] -= 1;

            let next_coins = coins + tokens[i][state[i]].1 - tokens[i][next_state[i]].1;

            if seen.insert(next_state) {
                todo.push(next_coins, next_state);
            }
        }
    }

    unreachable!()
}

fn unique(tokens: &[Vec<(i32, i32)>], state: &[usize]) -> bool {
    let mut seen = 0;

    for i in 0..6 {
        let mask = 1 << tokens[i][state[i]].0;

        if seen & mask != 0 {
            return false;
        }

        seen |= mask;
    }

    true
}
