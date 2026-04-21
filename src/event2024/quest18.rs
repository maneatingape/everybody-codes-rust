use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn part1(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let starts = [Point::new(0, 1)];
    bfs(&grid, &starts)
}

pub fn part2(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let starts = [Point::new(0, 1), Point::new(grid.width - 1, grid.height - 2)];
    bfs(&grid, &starts)
}

pub fn part3(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let distance = &mut grid.same_size_with(0);

    for point in grid.points().filter(|&p| grid[p] == b'P') {
        flood_fill(&grid, distance, point);
    }

    let start = grid.points().filter(|&p| grid[p] == b'.').min_by_key(|&p| distance[p]).unwrap();
    flood_fill(&grid, distance, start)
}

fn bfs(grid: &Grid<u8>, starts: &[Point]) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);
    let mut remaining = grid.bytes.iter().filter(|&&b| b == b'P').count();

    for &start in starts {
        todo.push_back((start, 0));
        seen[start] = true;
    }

    while let Some((point, cost)) = todo.pop_front() {
        if grid[point] == b'P' {
            remaining -= 1;
            if remaining == 0 {
                return cost;
            }
        }

        for next in neighbors(grid, point) {
            if !seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = true;
            }
        }
    }

    unreachable!()
}

fn flood_fill(grid: &Grid<u8>, distance: &mut Grid<u32>, start: Point) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);
    let mut total = 0;

    todo.push_back((start, 0));
    seen[start] = true;

    while let Some((point, cost)) = todo.pop_front() {
        distance[point] += cost;
        if grid[point] == b'P' {
            total += cost;
        }

        for next in neighbors(grid, point) {
            if !seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = true;
            }
        }
    }

    total
}

fn neighbors(grid: &Grid<u8>, point: Point) -> impl Iterator<Item = Point> {
    ORTHOGONAL
        .iter()
        .map(move |&offset| point + offset)
        .filter(|&next| grid.contains(next) && grid[next] != b'#')
}
