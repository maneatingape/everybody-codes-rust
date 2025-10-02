use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn part1(notes: &str) -> u32 {
    dig(notes, &ORTHOGONAL)
}

pub fn part2(notes: &str) -> u32 {
    dig(notes, &ORTHOGONAL)
}

pub fn part3(notes: &str) -> u32 {
    dig(notes, &DIAGONAL)
}

fn dig(notes: &str, neighbors: &[Point]) -> u32 {
    let grid = Grid::parse(notes);
    let mut depth = grid.same_size_with(0);
    let mut todo = VecDeque::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            if grid[p] == b'#' {
                todo.push_back(p);
            }
        }
    }

    while let Some(p) = todo.pop_front() {
        if neighbors
            .iter()
            .all(|&n| depth[p] <= if depth.contains(p + n) { depth[p + n] } else { 0 })
        {
            depth[p] += 1;
            todo.push_back(p);
        }
    }

    depth.bytes.iter().sum()
}
