use crate::util::grid::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::{BTreeSet, HashMap, VecDeque};

pub fn part1(notes: &str) -> i32 {
    solve(notes)
}

pub fn part2(notes: &str) -> i32 {
    solve(notes)
}

pub fn part3(notes: &str) -> i32 {
    solve(notes)
}

pub fn solve(notes: &str) -> i32 {
    let turns = notes.bytes().filter(u8::is_ascii_uppercase);
    let amounts = notes.iter_signed::<i32>();

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
    let shrink_x: HashMap<_, _> = xs.iter().enumerate().map(|(i, &x)| (x, i as i32)).collect();
    let shrink_y: HashMap<_, _> = ys.iter().enumerate().map(|(i, &y)| (y, i as i32)).collect();
    let mut grid = Grid::new(xs.len() as i32, ys.len() as i32, b'.');

    for (from, to) in walls {
        let (x1, x2) = (shrink_x[&from.x], shrink_x[&to.x]);
        let (y1, y2) = (shrink_y[&from.y], shrink_y[&to.y]);

        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[Point::new(x, y)] = b'#';
            }
        }
    }

    let start = Point::new(shrink_x[&0], shrink_y[&0]);
    let end = Point::new(shrink_x[&end.x], shrink_y[&end.y]);
    let mut todo = VecDeque::from([(start, 0)]);

    while let Some((from, cost)) = todo.pop_front() {
        for to in ORTHOGONAL.map(|o| o + from) {
            if grid.contains(to) {
                let next_cost = cost
                    + (xs[from.x as usize] - xs[to.x as usize]).abs()
                    + (ys[from.y as usize] - ys[to.y as usize]).abs();
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
