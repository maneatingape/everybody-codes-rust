use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> i32 {
    targets(notes)
}

pub fn part2(notes: &str) -> i32 {
    targets(notes)
}

pub fn part3(notes: &str) -> i32 {
    notes.iter_signed::<i32>().chunk::<2>().map(|[x, y]| ranking(x / 2, y - x / 2 - x % 2)).sum()
}

fn targets(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    grid.points()
        .map(|p| match grid[p] {
            b'T' => ranking(p.x - 1, grid.height - 2 - p.y),
            b'H' => 2 * ranking(p.x - 1, grid.height - 2 - p.y),
            _ => 0,
        })
        .sum()
}

fn ranking(x: i32, y: i32) -> i32 {
    (0..3)
        .find_map(|base| {
            let y = y - base;
            let horizontal = x + y;

            if x < y {
                None
            } else if x <= 2 * y {
                Some((base + 1) * y)
            } else if horizontal % 3 == 0 {
                Some((base + 1) * (horizontal / 3))
            } else {
                None
            }
        })
        .unwrap()
}
