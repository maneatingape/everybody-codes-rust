use crate::util::grid::*;
use crate::util::point::*;
use std::mem::swap;

const DIAGONAL: [Point; 4] =
    [Point::new(-1, -1), Point::new(1, -1), Point::new(-1, 1), Point::new(1, 1)];

const ITERATIONS: u64 = 1_000_000_000;
const PERIOD: u64 = 4095;
const FULL: u64 = ITERATIONS / PERIOD;
const PARTIAL: u64 = ITERATIONS % PERIOD + 1;

pub fn part1(notes: &str) -> u64 {
    let mut grid = Grid::parse(notes);
    let mut next = grid.same_size_with(b'.');
    (0..10).map(|_| step(&mut grid, &mut next)).sum()
}

pub fn part2(notes: &str) -> u64 {
    let mut grid = Grid::parse(notes);
    let mut next = grid.same_size_with(b'.');
    (0..2025).map(|_| step(&mut grid, &mut next)).sum()
}

pub fn part3(notes: &str) -> u64 {
    let pattern = Grid::parse(notes);
    let mut grid = Grid::new(34, 34, b'.');
    let mut next = grid.same_size_with(b'.');

    let mut round = || {
        let active = step(&mut grid, &mut next);
        matches(&grid, &pattern).then_some(active)
    };

    let first: u64 = (0..PARTIAL).filter_map(|_| round()).sum();
    let second: u64 = (PARTIAL..PERIOD).filter_map(|_| round()).sum();
    first + FULL * (first + second)
}

fn step(grid: &mut Grid<u8>, next: &mut Grid<u8>) -> u64 {
    let mut active = 0;

    for point in grid.points() {
        let count = DIAGONAL
            .iter()
            .map(|&d| d + point)
            .filter(|&p| grid.contains(p) && grid[p] == b'#')
            .count();

        let on = (grid[point] == b'#') != count.is_multiple_of(2);
        next[point] = if on { b'#' } else { b'.' };
        active += u64::from(on);
    }

    swap(grid, next);
    active
}

fn matches(grid: &Grid<u8>, pattern: &Grid<u8>) -> bool {
    let offset = Point::new((grid.width - pattern.width) / 2, (grid.height - pattern.height) / 2);
    (0..pattern.height / 2)
        .flat_map(|y| (0..pattern.width / 2).map(move |x| Point::new(x, y)))
        .all(|point| grid[point + offset] == pattern[point])
}
