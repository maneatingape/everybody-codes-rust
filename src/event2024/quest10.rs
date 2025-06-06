use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> String {
    let mut grid = Grid::parse(notes);
    let mut word = String::new();

    solve(&mut grid, ORIGIN);

    for y in 2..6 {
        for x in 2..6 {
            let point = Point::new(x, y);
            word.push(grid[point] as char);
        }
    }

    word
}

pub fn part2(notes: &str) -> i32 {
    notes
        .split("\n\n")
        .map(|row| {
            let mut grid = Grid::parse(row);
            (0..grid.width)
                .step_by(9)
                .filter_map(|x| solve(&mut grid, Point::new(x, 0)))
                .sum::<i32>()
        })
        .sum()
}

pub fn part3(notes: &str) -> i32 {
    let mut grid = Grid::parse(notes);
    let mut todo = Vec::new();
    let mut previous = -1;
    let mut total = 0;

    for y in (0..grid.height - 2).step_by(6) {
        for x in (0..grid.width - 2).step_by(6) {
            todo.push(Point::new(x, y));
        }
    }

    while previous < total {
        previous = total;

        todo.retain(|&corner| {
            if let Some(power) = solve(&mut grid, corner) {
                total += power;
                false
            } else {
                true
            }
        });
    }

    total
}

fn solve(grid: &mut Grid<u8>, corner: Point) -> Option<i32> {
    for y in 2..6 {
        for x in 2..6 {
            let mut row = 0;
            let mut col = 0;

            for i in 0..8 {
                row |= to_mask(grid[corner + Point::new(i, y)]);
                col |= to_mask(grid[corner + Point::new(x, i)]);
            }

            let unique = row & col;
            if unique.count_ones() == 1 {
                let point = corner + Point::new(x, y);
                grid[point] = to_ascii(unique);
            }
        }
    }

    let mut power = 0;
    let mut solved = true;

    for y in 2..6 {
        for x in 2..6 {
            let point = corner + Point::new(x, y);
            if grid[point].is_ascii_uppercase() {
                power += to_power(x, y, grid[point]);
                continue;
            }

            let mut unique = 0;
            let mut unknown = ORIGIN;
            let mut update = |point| match grid[point] {
                b'?' => unknown = point,
                other => unique ^= to_mask(other),
            };

            for i in 0..8 {
                update(corner + Point::new(i, y));
                update(corner + Point::new(x, i));
            }

            if unique.count_ones() == 1 {
                let ascii = to_ascii(unique);
                grid[point] = ascii;
                grid[unknown] = ascii;
                power += to_power(x, y, ascii);
            } else {
                solved = false;
            }
        }
    }

    solved.then_some(power)
}

fn to_mask(ascii: u8) -> u32 {
    if ascii.is_ascii_uppercase() { 1 << (ascii - b'A') } else { 0 }
}

fn to_ascii(mask: u32) -> u8 {
    b'A' + mask.trailing_zeros() as u8
}

fn to_power(x: i32, y: i32, ascii: u8) -> i32 {
    (4 * y + x - 9) * (ascii - b'A' + 1) as i32
}
