use std::collections::VecDeque;
use std::iter::from_fn;

use crate::util::grid::*;
use crate::util::point::*;

const FLY: [Point; 4] = [DOWN, LEFT, RIGHT, UP];
const TURN: [[usize; 3]; 4] = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];

pub fn part1(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();

    glide(&grid, start, 0, |_, _, time| time == 100)
        .map(|(_, altitude)| 1000 + altitude)
        .max()
        .unwrap()
}

pub fn part2(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();

    let mut todo = VecDeque::from([(start, 0, 0, 0, 0)]);
    let mut seen = grid.same_size_with([[i32::MIN; 4]; 4]);

    while let Some((point, direction, stage, altitude, time)) = todo.pop_front() {
        for (next, turn, altitude) in fly(&grid, point, direction, altitude) {
            if stage == 3 && next == start && altitude >= 0 {
                return time + 1;
            }

            let stage = stage + usize::from(b"ABC".get(stage) == Some(&grid[next]));

            if seen[next][stage][turn] < altitude {
                todo.push_back((next, turn, stage, altitude, time + 1));
                seen[next][stage][turn] = altitude;
            }
        }
    }

    unreachable!()
}

pub fn part3(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap().x as usize;

    let mut extended = grid.clone();
    extended.height += 1;
    extended.bytes.extend_from_slice(&grid.bytes[..grid.width as usize]);

    let size = grid.width as usize;
    let left = 1;
    let right = size - 1;

    // Calculate all possible combinations of altitude drops.
    let drop: Vec<_> = (left..right).map(|x| complete(&extended, x)).collect();

    let mut current = vec![i32::MIN; size];
    current[start] = 384400;

    for block in 0.. {
        let mut next = vec![i32::MIN; size];

        for (from, drops) in (left..right).zip(&drop) {
            for to in left..right {
                next[to] = next[to].max(current[from].saturating_add(drops[to]));
            }
        }

        if next.iter().all(|&a| a <= 0) {
            let complete = block * grid.height;
            let partial = (left..right)
                .filter(|&x| current[x] > 0)
                .map(|x| remainder(&extended, x, current[x]))
                .max()
                .unwrap_or(0);

            return complete + partial;
        }

        current = next;
    }

    unreachable!()
}

fn complete(grid: &Grid<u8>, start: usize) -> Vec<i32> {
    let start = Point::new(start as i32, 0);
    let mut result = vec![i32::MIN; grid.width as usize];

    for (point, altitude) in glide(grid, start, 0, |point, _, _| point.y == grid.height - 1) {
        let x = point.x as usize;
        result[x] = result[x].max(altitude);
    }

    result
}

fn remainder(grid: &Grid<u8>, start: usize, height: i32) -> i32 {
    let start = Point::new(start as i32, 0);

    glide(grid, start, height, |_, altitude, _| altitude <= 0)
        .map(|(point, _)| point.y)
        .max()
        .unwrap_or(0)
}

fn glide(
    grid: &Grid<u8>,
    start: Point,
    altitude: i32,
    done: impl Fn(Point, i32, i32) -> bool,
) -> impl Iterator<Item = (Point, i32)> {
    let mut todo = VecDeque::from([(start, 0, altitude, 0)]);
    let mut seen = grid.same_size_with([i32::MIN; 4]);

    from_fn(move || {
        while let Some((point, direction, altitude, time)) = todo.pop_front() {
            if done(point, altitude, time) {
                return Some((point, altitude));
            }

            for (next, turn, altitude) in fly(grid, point, direction, altitude) {
                if seen[next][turn] < altitude {
                    todo.push_back((next, turn, altitude, time + 1));
                    seen[next][turn] = altitude;
                }
            }
        }

        None
    })
}

fn fly(
    grid: &Grid<u8>,
    point: Point,
    direction: usize,
    altitude: i32,
) -> impl Iterator<Item = (Point, usize, i32)> {
    TURN[direction]
        .into_iter()
        .map(move |turn| (point + FLY[turn], turn))
        .filter(move |&(next, _)| grid.contains(next) && grid[next] != b'#')
        .map(move |(next, turn)| (next, turn, altitude + lift(grid[next])))
}

fn lift(tile: u8) -> i32 {
    match tile {
        b'+' => 1,
        b'-' => -2,
        _ => -1,
    }
}
