use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> i32 {
    targets(notes)
}

pub fn part2(notes: &str) -> i32 {
    targets(notes)
}

pub fn part3(notes: &str) -> i32 {
    notes.iter_signed::<i32>().chunk::<2>().map(|[x, y]| ranking(x / 2, y - x / 2 - x % 2)).sum()
}

pub fn targets(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            total += match grid[Point::new(x, y)] {
                b'T' => ranking(x - 1, grid.height - 2 - y),
                b'H' => 2 * ranking(x - 1, grid.height - 2 - y),
                _ => 0,
            }
        }
    }

    total
}

fn ranking(x: i32, y: i32) -> i32 {
    for base in 0..3 {
        let y = y - base;
        let horizontal = x + y;

        if x < y {
            continue;
        }
        if x <= 2 * y {
            return (base + 1) * y;
        }
        if horizontal % 3 == 0 {
            return (base + 1) * (horizontal / 3);
        }
    }

    unreachable!()
}
