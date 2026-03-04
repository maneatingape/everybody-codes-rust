use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn part1(notes: &str) -> u32 {
    let mut grid = Grid::parse(notes);
    let wave = grid.find(b'@').unwrap();
    let bone = grid.find(b'#').unwrap();

    let mut steps = 0;
    let mut position = wave;
    let mut direction = UP;

    while position != bone {
        grid[position] = b'+';

        let next = position + direction;
        direction = direction.clockwise();

        if grid[next] != b'+' {
            position = next;
            steps += 1;
        }
    }

    steps
}

pub fn part2(notes: &str) -> u32 {
    let mut grid = padded_grid(notes);
    let wave = grid.find(b'@').unwrap();
    let bone = grid.find(b'#').unwrap();

    let mut steps = 0;
    let mut position = wave;
    let mut direction = UP;

    while ORTHOGONAL.iter().any(|&o| grid[bone + o] == b'.') {
        grid[position] = b'+';

        for neighbor in ORTHOGONAL.map(|o| position + o) {
            if ORTHOGONAL.iter().all(|&o| grid[neighbor + o] != b'.') {
                grid[neighbor] = b'+';
            }
        }

        let next = position + direction;
        direction = direction.clockwise();

        if grid[next] == b'.' {
            position = next;
            steps += 1;
        }
    }

    steps
}

pub fn part3(notes: &str) -> u32 {
    let mut grid = padded_grid(notes);
    let start = grid.find(b'@').unwrap();
    grid[start] = b'+';

    let mut steps = 0;
    let mut position = start;
    let mut direction =
        [UP, UP, UP, RIGHT, RIGHT, RIGHT, DOWN, DOWN, DOWN, LEFT, LEFT, LEFT].into_iter().cycle();

    let mut fill = [b'a', b'b'].into_iter().cycle();
    let mut current_fill = b'.';

    while grid.points().any(|point| {
        grid[point] == b'#' && ORTHOGONAL.iter().any(|&o| grid[point + o] == current_fill)
    }) {
        let next = position + direction.next().unwrap();
        if grid[next] != current_fill {
            continue;
        }

        position = next;
        grid[position] = b'+';
        steps += 1;

        let previous_fill = current_fill;
        current_fill = fill.next().unwrap();
        flood_fill(&mut grid, previous_fill, current_fill);

        for byte in &mut grid.bytes {
            if *byte == previous_fill {
                *byte = b'+';
            }
        }
    }

    steps
}

fn padded_grid(notes: &str) -> Grid<u8> {
    let grid = Grid::parse(notes);
    let width = grid.width;
    let height = grid.height;
    let offset = Point::new(width, height);
    let mut padded = Grid::new(3 * width, 3 * height, b'.');

    for point in grid.points() {
        padded[point + offset] = grid[point];
    }

    padded
}

fn flood_fill(grid: &mut Grid<u8>, from: u8, to: u8) {
    grid[ORIGIN] = to;
    let mut todo = VecDeque::from([ORIGIN]);

    while let Some(point) = todo.pop_front() {
        for next in ORTHOGONAL.map(|o| point + o) {
            if grid.contains(next) && grid[next] == from {
                grid[next] = to;
                todo.push_back(next);
            }
        }
    }
}
