use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;
use std::iter::repeat_n;

const FLY: [Point; 4] = [DOWN, LEFT, RIGHT, UP];
const TURN: [[usize; 3]; 4] = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];

pub fn part1(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();

    let mut todo = VecDeque::from([(start, 0, 0, 0)]);
    let mut seen = minimum_from(&grid);
    let mut result = 0;

    while let Some((point, direction, altitude, time)) = todo.pop_front() {
        if time == 100 {
            result = result.max(altitude);
            continue;
        }

        for turn in TURN[direction] {
            let next = point + FLY[turn];

            if grid.contains(next) && grid[next] != b'#' {
                let altitude = match grid[next] {
                    b'+' => altitude + 1,
                    b'-' => altitude - 2,
                    _ => altitude - 1,
                };
                if seen[next][turn] < altitude {
                    todo.push_back((next, turn, altitude, time + 1));
                    seen[next][turn] = altitude;
                }
            }
        }
    }

    1000 + result
}

pub fn part2(notes: &str) -> i32 {
    let mut grid = Grid::parse(notes);
    let mut waypoints = [b'S', b'A', b'B', b'C'].map(|p| grid.find(p).unwrap());
    let mut total = 0;

    for _ in 0..4 {
        grid[waypoints[0]] = b'#';
        grid[waypoints[1]] = b'.';
        grid[waypoints[2]] = b'#';
        grid[waypoints[3]] = b'#';

        total += segment(&grid, waypoints[0], waypoints[1]);
        waypoints.rotate_left(1);
    }

    total
}

#[expect(clippy::needless_range_loop)]
pub fn part3(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let start = grid.find(b'S').unwrap();

    let mut extended = grid.clone();
    extended.height += 1;
    extended.bytes.extend_from_slice(&grid.bytes[0..grid.width as usize]);

    let start = start.x as usize;
    let size = grid.width as usize;
    let left = 1;
    let right = size - 1;

    // Caculate all possible combinations of altitude drops.
    let mut drop = vec![vec![]; size];

    for from in left..right {
        drop[from] = complete(&extended, from);
    }

    let mut current = vec![i32::MIN; size];
    current[start] = 384400;

    for block in 0.. {
        let mut next = vec![i32::MIN; size];

        for from in left..right {
            for to in left..right {
                let candidate = current[from].saturating_add(drop[from][to]);
                next[to] = next[to].max(candidate);
            }
        }

        if next.iter().all(|&a| a <= 0) {
            let complete = block * grid.height;
            let mut partial = 0;

            for x in left..right {
                if current[x] > 0 {
                    partial = partial.max(remainder(&grid, x, current[x]));
                }
            }

            return complete + partial;
        }

        current = next;
    }

    unreachable!()
}

fn segment(grid: &Grid<u8>, start: Point, end: Point) -> i32 {
    let mut todo = VecDeque::from([(start, 0, 0, 0)]);
    let mut seen = minimum_from(grid);

    while let Some((point, direction, altitude, time)) = todo.pop_front() {
        if point == end {
            if altitude >= 0 {
                return time;
            }
            continue;
        }

        for turn in TURN[direction] {
            let next = point + FLY[turn];

            if grid.contains(next) && grid[next] != b'#' {
                let altitude = match grid[next] {
                    b'+' => altitude + 1,
                    b'-' => altitude - 2,
                    _ => altitude - 1,
                };
                if seen[next][turn] < altitude {
                    todo.push_back((next, turn, altitude, time + 1));
                    seen[next][turn] = altitude;
                }
            }
        }
    }

    unreachable!()
}

fn complete(grid: &Grid<u8>, start: usize) -> Vec<i32> {
    let start = Point::new(start as i32, 0);

    let mut todo = VecDeque::from([(start, 0, 0, 0)]);
    let mut seen = minimum_from(grid);
    let mut result = vec![i32::MIN; grid.width as usize];

    while let Some((point, direction, altitude, time)) = todo.pop_front() {
        if point.y == grid.height - 1 {
            let index = point.x as usize;
            result[index] = result[index].max(altitude);
            continue;
        }

        for turn in TURN[direction] {
            let next = point + FLY[turn];

            if grid.contains(next) && grid[next] != b'#' {
                let altitude = match grid[next] {
                    b'+' => altitude + 1,
                    b'-' => altitude - 2,
                    _ => altitude - 1,
                };
                if seen[next][turn] < altitude {
                    todo.push_back((next, turn, altitude, time + 1));
                    seen[next][turn] = altitude;
                }
            }
        }
    }

    result
}

fn remainder(grid: &Grid<u8>, start: usize, height: i32) -> i32 {
    let start = Point::new(start as i32, 0);
    let mut todo = VecDeque::from([(start, 0, height, 0)]);
    let mut seen = minimum_from(grid);
    let mut result = 0;

    while let Some((point, direction, altitude, time)) = todo.pop_front() {
        if altitude <= 0 {
            result = result.max(point.y);
            continue;
        }

        for turn in TURN[direction] {
            let next = point + FLY[turn];

            if grid.contains(next) && grid[next] != b'#' {
                let altitude = match grid[next] {
                    b'+' => altitude + 1,
                    b'-' => altitude - 2,
                    _ => altitude - 1,
                };
                if seen[next][turn] < altitude {
                    todo.push_back((next, turn, altitude, time + 1));
                    seen[next][turn] = altitude;
                }
            }
        }
    }

    result
}

fn minimum_from(grid: &Grid<u8>) -> Grid<[i32; 4]> {
    Grid {
        width: grid.width,
        height: grid.height,
        bytes: repeat_n([i32::MIN; 4], (grid.width * grid.height) as usize).collect(),
    }
}
