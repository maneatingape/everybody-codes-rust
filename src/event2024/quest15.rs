use crate::util::grid::*;
use crate::util::point::*;
use std::collections::{HashSet, VecDeque};

const OPEN: u32 = 0;
const WALL: u32 = u32::MAX;

pub fn part1(notes: &str) -> u32 {
    solve(notes)
}

pub fn part2(notes: &str) -> u32 {
    solve(notes)
}

pub fn part3(notes: &str) -> u32 {
    solve(notes)
}

fn solve(notes: &str) -> u32 {
    let ascii = Grid::parse(notes);
    let start = Point::new(ascii.width / 2, 0);

    let mut grid = ascii.default_copy();
    grid.bytes = ascii.bytes.into_iter().map(to_mask).collect();

    let (total, _) = collect(&mut grid, &mut HashSet::from([start]), start);
    total
}

fn collect(grid: &mut Grid<u32>, seen: &mut HashSet<Point>, start: Point) -> (u32, u32) {
    // Flood fill
    let mut todo = VecDeque::from([start]);
    let mut children = Vec::new();

    while let Some(point) = todo.pop_front() {
        for next in neighbours(grid, point) {
            if seen.insert(next) {
                if grid[next] == OPEN {
                    todo.push_back(next);
                } else {
                    children.push(next);
                }
            }
        }
    }

    // Process sub-graphs
    let mut total = 0;
    let mut letters = 0;

    for child in children {
        let (child_total, child_letters) = collect(grid, seen, child);
        total += child_total;
        letters |= child_letters & !grid[start];
    }

    // Process current graph
    let mut todo = VecDeque::from([(start, letters, 0)]);
    let mut seen = HashSet::from([(start, letters)]);

    while let Some((point, remaining, cost)) = todo.pop_front() {
        if point == start && remaining == 0 {
            total += cost;
            break;
        }

        for next in neighbours(grid, point) {
            let remaining = remaining & !grid[next];
            if seen.insert((next, remaining)) {
                todo.push_back((next, remaining, cost + 1));
            }
        }
    }

    grid[start] |= letters;
    (total, grid[start])
}

fn neighbours(grid: &Grid<u32>, point: Point) -> impl Iterator<Item = Point> + '_ {
    ORTHOGONAL
        .iter()
        .map(move |&offset| point + offset)
        .filter(|&next| grid.contains(next) && grid[next] != WALL)
}

fn to_mask(ascii: u8) -> u32 {
    match ascii {
        b'.' => OPEN,
        b'#' | b'~' => WALL,
        _ => 1 << (ascii - b'A'),
    }
}
