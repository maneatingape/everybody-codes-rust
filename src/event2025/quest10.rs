use crate::util::grid::*;
use crate::util::point::*;
use std::collections::{HashMap, HashSet};

const MOVES: [Point; 8] = [
    Point::new(-1, -2),
    Point::new(1, -2),
    Point::new(-2, 1),
    Point::new(-2, -1),
    Point::new(2, 1),
    Point::new(2, -1),
    Point::new(-1, 2),
    Point::new(1, 2),
];

struct State {
    grid: Grid<u8>,
    sheep_cache: HashMap<(Point, [u8; 8]), u64>,
    dragon_cache: HashMap<(Point, [u8; 8]), u64>,
}

pub fn part1(notes: &str) -> usize {
    part1_testable(notes, 4)
}

pub fn part1_testable(notes: &str, rounds: i32) -> usize {
    let grid = Grid::parse(notes);
    let dragon = grid.find(b'D').unwrap();
    let mut dragons = HashSet::from([dragon]);

    for _ in 0..rounds {
        dragons = dragons
            .iter()
            .flat_map(|&from| MOVES.map(|delta| from + delta))
            .chain(dragons.iter().copied())
            .collect();
    }

    dragons.iter().filter(|&&p| grid[p] == b'S').count()
}

pub fn part2(notes: &str) -> usize {
    part2_testable(notes, 20)
}

pub fn part2_testable(notes: &str, rounds: i32) -> usize {
    let mut grid = Grid::parse(notes);
    let dragon = grid.find(b'D').unwrap();
    let mut dragons = HashSet::from([dragon]);

    for round in 0..rounds {
        dragons = dragons.iter().flat_map(|&from| MOVES.map(|delta| from + delta)).collect();

        for &dragon in &dragons {
            if grid[dragon] != b'#' {
                for sheep in [dragon + UP * round, dragon + UP * (round + 1)] {
                    if grid.contains(sheep) && grid[sheep] == b'S' {
                        grid[sheep] = b'X';
                    }
                }
            }
        }
    }

    grid.bytes.iter().filter(|&&b| b == b'X').count()
}

pub fn part3(notes: &str) -> u64 {
    let mut grid = Grid::parse(notes);
    let dragon = grid.find(b'D').unwrap();
    let mut sheep = [u8::MAX; 8];

    for x in 0..grid.width {
        if grid[Point::new(x, 0)] == b'S' {
            sheep[x as usize] = 0;
        }

        let mut point = Point::new(x, grid.height - 1);
        while grid[point] == b'#' {
            grid[point] = b'H';
            point += UP;
        }
    }

    let state = &mut State { grid, sheep_cache: HashMap::new(), dragon_cache: HashMap::new() };
    sheep_move(state, dragon, sheep)
}

fn sheep_move(state: &mut State, dragon: Point, sheep: [u8; 8]) -> u64 {
    if let Some(&value) = state.sheep_cache.get(&(dragon, sheep)) {
        return value;
    }

    let (result, moved) = sheep.iter().enumerate().fold((0, false), |(result, moved), (i, &s)| {
        if s == u8::MAX {
            (result, moved)
        } else {
            let point = Point::new(i as i32, (s + 1) as i32);

            if !state.grid.contains(point) || state.grid[point] == b'H' {
                (result, true)
            } else if point == dragon && state.grid[point] != b'#' {
                (result, moved)
            } else {
                let mut next = sheep;
                next[i] += 1;
                (result + dragon_move(state, dragon, next), true)
            }
        }
    });

    let result = if moved { result } else { dragon_move(state, dragon, sheep) };
    state.sheep_cache.insert((dragon, sheep), result);
    result
}

fn dragon_move(state: &mut State, dragon: Point, sheep: [u8; 8]) -> u64 {
    if let Some(&value) = state.dragon_cache.get(&(dragon, sheep)) {
        return value;
    }

    let result = MOVES
        .iter()
        .map(|&delta| {
            let dragon = dragon + delta;
            if !state.grid.contains(dragon) {
                return 0;
            }

            let mut next = sheep;
            if state.grid[dragon] != b'#' && next[dragon.x as usize] as i32 == dragon.y {
                next[dragon.x as usize] = u8::MAX;
            }

            if next.iter().all(|&b| b == u8::MAX) { 1 } else { sheep_move(state, dragon, next) }
        })
        .sum();

    state.dragon_cache.insert((dragon, sheep), result);
    result
}
