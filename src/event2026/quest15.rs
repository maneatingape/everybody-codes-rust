use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;

const TARGET: u64 = 202620262026;

pub fn part1(notes: &str) -> u64 {
    let (grid, start, end) = parse(notes);

    let mut all = 0;
    let mut sulphur = grid.same_size_with(0);

    for (i, point) in grid.points().filter(|&point| grid[point] == b'@').enumerate() {
        all |= 1 << i;
        sulphur[point] = 1 << i;
    }

    let mut todo = VecDeque::from([(start, RIGHT, 0, 0)]);
    let mut seen = HashSet::from([(start, RIGHT, 0)]);

    while let Some((position, direction, time, collected)) = todo.pop_front() {
        if position == end && collected == all {
            return time;
        }

        for (next_position, next_direction) in moves(&grid, position, direction) {
            let next_collected = collected | sulphur[next_position];

            if seen.insert((next_position, next_direction, next_collected)) {
                todo.push_back((next_position, next_direction, time + 1, next_collected));
            }
        }
    }

    unreachable!()
}

pub fn part2(notes: &str) -> u64 {
    let (grid, start, end) = parse(notes);

    let mut todo = VecDeque::from([(start, RIGHT, 0, 0)]);
    let mut seen = HashMap::from([((start, RIGHT), 0)]);

    while let Some((position, direction, time, sulphur)) = todo.pop_front() {
        if position == end && sulphur >= 2026 {
            return time;
        }

        for (next_position, next_direction) in moves(&grid, position, direction) {
            let next_sulphur = sulphur + u64::from(grid[next_position] == b'@');
            let key = (next_position, next_direction);

            if seen.get(&key).is_none_or(|&collected| collected < next_sulphur) {
                todo.push_back((next_position, next_direction, time + 1, next_sulphur));
                seen.insert(key, next_sulphur);
            }
        }
    }

    unreachable!()
}

pub fn part3(notes: &str) -> u64 {
    let (grid, start, end) = parse(notes);

    let neighbors = |point| ORTHOGONAL.iter().filter(|&&o| grid[point + o] != b'#').count();
    let mut cycles = Vec::new();

    for start in grid.points().filter(|&point| grid[point] == b'.' && neighbors(point) == 4) {
        cycles.extend(find_cycles(&grid, start, Point::clockwise));
        cycles.extend(find_cycles(&grid, start, Point::counter_clockwise));
    }

    let (cycle_start, cycle_time, cycle_sulphur) = cycles
        .into_iter()
        .min_by(|&(_, time1, sulphur1), &(_, time2, sulphur2)| {
            (time1 * sulphur2).cmp(&(time2 * sulphur1))
        })
        .unwrap();

    let mut todo = MinHeap::from([(0, (start, RIGHT, 0, 0, false))]);
    let mut seen = HashMap::new();

    while let Some((_, (position, direction, time, sulphur, in_cycle))) = todo.pop() {
        if position == end && in_cycle && (TARGET - sulphur).is_multiple_of(cycle_sulphur) {
            return time + cycle_time * (TARGET - sulphur) / cycle_sulphur;
        }

        for (next_position, next_direction) in moves(&grid, position, direction) {
            let next_time = time + 1;
            let next_sulphur = sulphur + u64::from(grid[next_position] == b'@');
            let next_in_cycle = in_cycle || next_position == cycle_start;

            let key = (next_position, next_direction, next_sulphur % cycle_sulphur, next_in_cycle);
            let next_cost = cycle_sulphur * next_time + cycle_time * (TARGET - next_sulphur);

            if seen.get(&key).is_none_or(|&cost| cost > next_cost) {
                todo.push(
                    next_cost,
                    (next_position, next_direction, next_time, next_sulphur, next_in_cycle),
                );
                seen.insert(key, next_cost);
            }
        }
    }

    unreachable!()
}

fn moves(
    grid: &Grid<u8>,
    position: Point,
    direction: Point,
) -> impl Iterator<Item = (Point, Point)> {
    [direction, direction.clockwise(), direction.counter_clockwise()]
        .into_iter()
        .map(move |next_direction| (position + next_direction, next_direction))
        .filter(move |&(next_position, _)| {
            grid.contains(next_position) && grid[next_position] != b'#'
        })
}

fn find_cycles(grid: &Grid<u8>, start: Point, turn: fn(Point) -> Point) -> Vec<(Point, u64, u64)> {
    let mut todo = VecDeque::from([(start, LEFT, 0, 0)]);
    let mut seen = HashSet::from([start + LEFT]);
    let mut cycles = Vec::new();

    while let Some((position, direction, time, sulphur)) = todo.pop_front() {
        for next_direction in [direction, turn(direction)] {
            let next_position = position + next_direction;
            let next_time = time + 1;
            let next_sulphur = sulphur + u64::from(grid[next_position] == b'@');

            if next_position == start {
                cycles.push((start, next_time, next_sulphur));
            } else if grid[next_position] != b'#' && seen.insert(next_position) {
                todo.push_back((next_position, next_direction, next_time, next_sulphur));
            }
        }
    }

    cycles
}

fn parse(notes: &str) -> (Grid<u8>, Point, Point) {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    (grid, start, end)
}
