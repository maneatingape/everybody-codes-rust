use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;
use std::collections::HashMap;

pub fn part1(notes: &str) -> u32 {
    dijkstra(notes, b'S', b'E')
}

pub fn part2(notes: &str) -> u32 {
    dijkstra(notes, b'S', b'E')
}

pub fn part3(notes: &str) -> u32 {
    dijkstra(notes, b'E', b'S')
}

fn dijkstra(notes: &str, start: u8, end: u8) -> u32 {
    let grid = Grid::parse(notes);
    let start = grid.find(start).unwrap();

    let mut todo = MinHeap::from([(0, start)]);
    let mut seen = HashMap::from([(start, 0)]);

    while let Some((cost, point)) = todo.pop() {
        if grid[point] == end {
            return cost;
        }

        for next in ORTHOGONAL.iter().map(|&o| point + o) {
            if grid.contains(next) && grid[next] != b'#' {
                let first = parse(grid[point]);
                let second = parse(grid[next]);
                let diff = first.abs_diff(second);
                let extra = diff.min(10 - diff);
                let cost = cost + extra + 1;

                if !seen.contains_key(&next) || seen[&next] > cost {
                    seen.insert(next, cost);
                    todo.push(cost, next);
                }
            }
        }
    }

    unreachable!()
}

fn parse(b: u8) -> u32 {
    if b.is_ascii_digit() { (b - b'0') as u32 } else { 0 }
}
