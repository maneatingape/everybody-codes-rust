use std::collections::HashSet;

use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> i32 {
    solve(notes)
}

pub fn part2(notes: &str) -> i32 {
    solve(notes)
}

pub fn part3(notes: &str) -> i32 {
    solve(notes)
}

fn solve(notes: &str) -> i32 {
    let mut grid = Grid::parse(notes);
    let points: Vec<_> = grid.points().collect();

    points
        .into_iter()
        .map(|start| {
            if grid[start] == b'.' {
                return 0;
            }

            let mut todo = vec![start];
            let mut block = HashSet::from([start]);

            while let Some(point) = todo.pop() {
                grid[point] = b'.';

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == b'#' && block.insert(next) {
                        todo.push(next);
                    }
                }
            }

            let (x0, x3, y0, y3) = bounds(&block);
            let gap: HashSet<_> = (y0..=y3)
                .flat_map(|y| (x0..=x3).map(move |x| Point::new(x, y)))
                .filter(|point| !block.contains(point))
                .collect();

            if gap.is_empty() {
                let (width, height) = (x3 - x0 + 1, y3 - y0 + 1);
                width.min(height)
            } else {
                let (x1, x2, y1, y2) = bounds(&gap);
                let (left, right, top, bottom) = (x1 - x0, x3 - x2, y1 - y0, y3 - y2);
                let (gap_width, gap_height) = (x2 - x1 + 1, y2 - y1 + 1);

                (left + right + top.min(gap_width) + bottom.min(gap_width))
                    .min(top + bottom + left.min(gap_height) + right.min(gap_height))
            }
        })
        .sum()
}

fn bounds(points: &HashSet<Point>) -> (i32, i32, i32, i32) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    (min_x, max_x, min_y, max_y)
}
