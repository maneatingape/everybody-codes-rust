use crate::util::parse::*;
use crate::util::point::*;
use std::collections::{BTreeSet, HashSet, VecDeque};

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

    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    let mut walls = Vec::new();

    for (turn, amount) in turns.zip(amounts) {
        direction =
            if turn == b'R' { direction.clockwise() } else { direction.counter_clockwise() };
        let start = end;
        end += direction * amount;

        let (x1, x2) = minmax(start.x, end.x);
        let (y1, y2) = minmax(start.y, end.y);

        xs.insert(x1 - 1);
        xs.insert(x2 + 1);
        ys.insert(y1 - 1);
        ys.insert(y2 + 1);

        if direction == LEFT || direction == RIGHT {
            walls.push([x1 + 1, x2 - 1, y1, y2]);
        } else {
            walls.push([x1, x2, y1 + 1, y2 - 1]);
        }
    }

    xs.insert(end.x);
    ys.insert(end.y);

    let mut todo = VecDeque::from([(ORIGIN, 0)]);
    let mut seen = HashSet::from([ORIGIN]);

    while let Some((from, cost)) = todo.pop_front() {
        if from == end {
            return cost;
        }

        let next = [
            xs.range(..from.x).next_back().map(|&x| Point::new(x, from.y)),
            xs.range(from.x + 1..).next().map(|&x| Point::new(x, from.y)),
            ys.range(..from.y).next_back().map(|&y| Point::new(from.x, y)),
            ys.range(from.y + 1..).next().map(|&y| Point::new(from.x, y)),
        ];

        for to in next.into_iter().flatten() {
            if !seen.contains(&to) && can_move(&walls, from, to) {
                todo.push_back((to, cost + from.manhattan(to)));
                seen.insert(to);
            }
        }
    }

    unreachable!()
}

fn minmax(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}

fn can_move(walls: &[[i32; 4]], from: Point, to: Point) -> bool {
    let (x1, x2) = minmax(from.x, to.x);
    let (y1, y2) = minmax(from.y, to.y);
    walls.iter().all(|&[x3, x4, y3, y4]| x1 > x4 || x2 < x3 || y1 > y4 || y2 < y3)
}
