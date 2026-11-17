use std::collections::{HashSet, VecDeque};

use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let end = Point::new(grid.width - 1, grid.height - 1);

    let mut todo = VecDeque::from([(ORIGIN, 0)]);
    let mut seen = HashSet::from([ORIGIN]);

    while let Some((position, cost)) = todo.pop_front() {
        if position == end {
            return cost;
        }

        for next in ORTHOGONAL.map(|o| position + o) {
            if !gap(&grid, next) && seen.insert(next) {
                todo.push_back((next, cost + 1));
            }
        }
    }

    unreachable!()
}

pub fn part2(notes: &str) -> usize {
    let grid = Grid::parse(notes);
    grid.points()
        .filter(|&point| grid[point] == b'#')
        .flat_map(|point| ORTHOGONAL.map(|o| point + o))
        .filter(|&next| gap(&grid, next))
        .count()
}

pub fn part3(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let corner = Point::new(grid.width - 1, grid.height - 1);

    let start = [(ORIGIN, UP), (ORIGIN, LEFT)];
    let end = [(corner, RIGHT), (corner, DOWN)];

    let mut todo = VecDeque::from(start.map(|state| (state, 1)));
    let mut seen = HashSet::from(start);

    while let Some((state @ (position, direction), cost)) = todo.pop_front() {
        if end.contains(&state) {
            return cost;
        }

        for turn in [direction.clockwise(), direction.counter_clockwise()] {
            let side = position + turn;
            let diagonal = side + direction;

            let next = if gap(&grid, side) {
                (position, turn)
            } else if gap(&grid, diagonal) {
                (side, direction)
            } else {
                (diagonal, turn * -1)
            };

            if seen.insert(next) {
                todo.push_back((next, cost + 1));
            }
        }
    }

    unreachable!()
}

fn gap(grid: &Grid<u8>, point: Point) -> bool {
    !grid.contains(point) || grid[point] == b'.'
}
