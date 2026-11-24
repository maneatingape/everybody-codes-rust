use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> i32 {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);
    suffix.lines().zip(0..).map(|(behavior, slot)| drop(&grid, behavior, slot)).sum()
}

pub fn part2(notes: &str) -> i32 {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);
    suffix
        .lines()
        .map(|behavior| {
            (0..(grid.width + 1) / 2).map(|slot| drop(&grid, behavior, slot)).max().unwrap()
        })
        .sum()
}

pub fn part3(notes: &str) -> String {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let grid = Grid::parse(prefix);
    let behaviors: Vec<_> = suffix.lines().collect();

    let size = 1 << behaviors.len();
    let mut min = vec![1_000_000; size];
    let mut max = vec![-1_000_000; size];
    min[0] = 0;
    max[0] = 0;

    for slot in 0..(grid.width + 1) / 2 {
        let coins: Vec<_> = behaviors.iter().map(|behavior| drop(&grid, behavior, slot)).collect();

        for mask in (0..size).rev() {
            for (token, coin) in coins.iter().enumerate() {
                let next = mask | (1 << token);
                if next != mask {
                    min[next] = min[next].min(min[mask] + coin);
                    max[next] = max[next].max(max[mask] + coin);
                }
            }
        }
    }

    format!("{} {}", min[size - 1], max[size - 1])
}

fn drop(grid: &Grid<u8>, behavior: &str, slot: i32) -> i32 {
    let mut x = slot * 2;
    let mut directions = behavior.bytes();

    for y in 0..grid.height {
        if grid[Point::new(x, y)] == b'*' {
            x += if directions.next().unwrap() == b'R' { 1 } else { -1 };

            if x < 0 {
                x += 2;
            }
            if x >= grid.width {
                x -= 2;
            }
        }
    }

    (x - slot + 1).max(0)
}
