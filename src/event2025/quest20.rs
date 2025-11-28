use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn part1(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let mut result = 0;

    for y in 0..grid.height - 1 {
        for x in 0..grid.width - 1 {
            let point = Point::new(x, y);
            if grid[point] == b'T' {
                result += u32::from(grid[point + RIGHT] == b'T');
                result += u32::from(grid[point + DOWN] == b'T' && odd_parity(point));
            }
        }
    }

    result
}

pub fn part2(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();

    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);

    todo.push_back((start, 0));
    seen[start] = true;

    while let Some((point, cost)) = todo.pop_front() {
        if grid[point] == b'E' {
            return cost;
        }

        let moves = [
            (point + UP, even_parity(point)),
            (point + DOWN, odd_parity(point)),
            (point + LEFT, true),
            (point + RIGHT, true),
        ];

        for (next, allowed) in moves {
            if allowed && grid.contains(next) && grid[next].is_ascii_uppercase() && !seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = true;
            }
        }
    }

    unreachable!()
}

pub fn part3(notes: &str) -> usize {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();

    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with([false; 3]);

    todo.push_back((start, 0));
    seen[start][0] = true;

    while let Some((point, cost)) = todo.pop_front() {
        if grid[point] == b'E' {
            return cost;
        }

        let point = rotate_anticlockwise(&grid, point);
        let cost = cost + 1;
        let moves = [
            (point + UP, even_parity(point)),
            (point + DOWN, odd_parity(point)),
            (point + LEFT, true),
            (point + RIGHT, true),
            (point, true),
        ];

        for (next, allowed) in moves {
            if allowed
                && grid.contains(next)
                && grid[next].is_ascii_uppercase()
                && !seen[next][cost % 3]
            {
                todo.push_back((next, cost));
                seen[next][cost % 3] = true;
            }
        }
    }

    unreachable!()
}

fn odd_parity(point: Point) -> bool {
    (point.x + point.y) % 2 == 1
}

fn even_parity(point: Point) -> bool {
    (point.x + point.y) % 2 == 0
}

fn rotate_anticlockwise(grid: &Grid<u8>, point: Point) -> Point {
    let next = Point::new(
        i32::midpoint(grid.width - point.x, point.y) + point.y,
        grid.height - 1 - (point.x - point.y) / 2 - point.y,
    );
    if even_parity(point) { next } else { next + UP }
}
