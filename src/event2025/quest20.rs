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
    let mut grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();
    let mut todo = VecDeque::from([(start, 0)]);

    while let Some((point, cost)) = todo.pop_front() {
        let moves = [if odd_parity(point) { DOWN } else { UP }, LEFT, RIGHT];

        for next in moves.map(|m| point + m) {
            if grid.contains(next) && grid[next].is_ascii_uppercase() {
                if grid[next] == b'E' {
                    return cost + 1;
                }
                grid[next] = b'#';
                todo.push_back((next, cost + 1));
            }
        }
    }

    unreachable!()
}

pub fn part3(notes: &str) -> usize {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();
    let mut layers = [grid.clone(), grid.clone(), grid.clone()];
    let mut todo = VecDeque::from([(start, 0)]);

    while let Some((point, cost)) = todo.pop_front() {
        let layer = &mut layers[(cost + 1) % 3];
        let jump = rotate_anticlockwise(layer, point);
        let moves = [if odd_parity(jump) { DOWN } else { UP }, LEFT, RIGHT, ORIGIN];

        for next in moves.map(|m| jump + m) {
            if layer.contains(next) && layer[next].is_ascii_uppercase() {
                if layer[next] == b'E' {
                    return cost + 1;
                }
                layer[next] = b'#';
                todo.push_back((next, cost + 1));
            }
        }
    }

    unreachable!()
}

fn odd_parity(point: Point) -> bool {
    (point.x + point.y) % 2 == 1
}

fn rotate_anticlockwise(grid: &Grid<u8>, point: Point) -> Point {
    let half = grid.width / 2;
    Point::new(
        half - (point.x - point.y) / 2 + point.y,
        half - (point.x - point.y + 1) / 2 - point.y,
    )
}
