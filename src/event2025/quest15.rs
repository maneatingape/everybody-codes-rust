use std::collections::{BTreeSet, VecDeque};

use crate::util::grid::*;
use crate::util::integer::*;
use crate::util::parse::*;
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
    let turns = notes.bytes().filter(u8::is_ascii_uppercase);
    let amounts = notes.iter_signed();

    let mut direction = UP;
    let mut end = ORIGIN;

    let mut walls = Vec::new();
    let mut xs = BTreeSet::from([-1, 0, 1]);
    let mut ys = BTreeSet::from([-1, 0, 1]);

    for (turn, amount) in turns.zip(amounts) {
        direction =
            if turn == b'R' { direction.clockwise() } else { direction.counter_clockwise() };
        let start = end;
        end += direction * amount;

        walls.push((start, end));
        xs.extend([end.x - 1, end.x, end.x + 1]);
        ys.extend([end.y - 1, end.y, end.y + 1]);
    }

    let xs: Vec<_> = xs.into_iter().collect();
    let ys: Vec<_> = ys.into_iter().collect();
    let shrink = |p: Point| {
        let x = xs.binary_search(&p.x).unwrap();
        let y = ys.binary_search(&p.y).unwrap();
        Point::new(x as i32, y as i32)
    };
    let expand = |p: Point| Point::new(xs[p.x as usize], ys[p.y as usize]);
    let mut grid = Grid::new(xs.len() as i32, ys.len() as i32, b'.');

    for (from, to) in walls {
        let (from, to) = (shrink(from), shrink(to));
        let (x1, x2) = from.x.minmax(to.x);
        let (y1, y2) = from.y.minmax(to.y);

        for y in y1..=y2 {
            for x in x1..=x2 {
                grid[Point::new(x, y)] = b'#';
            }
        }
    }

    let start = shrink(ORIGIN);
    let end = shrink(end);
    let mut todo = VecDeque::from([(start, 0)]);

    while let Some((from, cost)) = todo.pop_front() {
        for to in ORTHOGONAL.map(|o| o + from) {
            if grid.contains(to) {
                let next_cost = cost + expand(from).manhattan(expand(to));
                if grid[to] != b'#' {
                    grid[to] = b'#';
                    todo.push_back((to, next_cost));
                } else if to == end {
                    return next_cost;
                }
            }
        }
    }

    unreachable!()
}
