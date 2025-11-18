use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn part1(notes: &str) -> usize {
    let grid = Grid::parse(notes);
    let first = ignite(&grid, ORIGIN);

    count(&first.bytes)
}

pub fn part2(notes: &str) -> usize {
    let grid = Grid::parse(notes);
    let mut first = ignite(&grid, ORIGIN);
    let second = ignite(&grid, Point::new(grid.width - 1, grid.height - 1));

    union(&mut first.bytes, &second.bytes);
    count(&first.bytes)
}

pub fn part3(notes: &str) -> usize {
    let grid = Grid::parse(notes);
    let mut sets = Vec::new();
    let mut points: Vec<_> =
        (0..grid.height).flat_map(|y| (0..grid.width).map(move |x| Point::new(x, y))).collect();

    points.sort_unstable_by_key(|&p| grid[p]);

    while let Some(start) = points.pop() {
        let set = ignite(&grid, start);
        points.retain(|&p| !set[p]);
        sets.push(set.bytes);
    }

    sets.sort_by_cached_key(|s| count(s));
    let first = sets.pop().unwrap();

    sets.iter_mut().for_each(|s| union(s, &first));
    sets.sort_by_cached_key(|s| count(s));
    let second = sets.pop().unwrap();

    sets.iter_mut().for_each(|s| union(s, &second));
    sets.sort_by_cached_key(|s| count(s));
    let third = sets.pop().unwrap();

    count(&third)
}

fn ignite(grid: &Grid<u8>, start: Point) -> Grid<bool> {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);

    todo.push_back(start);
    seen[start] = true;

    while let Some(point) = todo.pop_front() {
        for next in ORTHOGONAL.map(|o| point + o) {
            if grid.contains(next) && grid[next] <= grid[point] && !seen[next] {
                todo.push_back(next);
                seen[next] = true;
            }
        }
    }

    seen
}

fn count(this: &[bool]) -> usize {
    this.iter().filter(|&&b| b).count()
}

fn union(this: &mut [bool], other: &[bool]) {
    this.iter_mut().zip(other).for_each(|(a, &b)| *a = *a || b);
}
