use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> String {
    let mut grid = Grid::parse(notes);
    let mut word = String::new();

    solve_both(&mut grid, ORIGIN);

    for y in 2..6 {
        for x in 2..6 {
            let point = Point::new(x, y);
            word.push(grid[point] as char);
        }
    }

    word
}

pub fn part2(notes: &str) -> u32 {
    let mut total = 0;

    for row in notes.split("\n\n") {
        let mut grid = Grid::parse(row);

        for x in (0..grid.width).step_by(9) {
            let corner = Point::new(x, 0);
            solve_both(&mut grid, corner);
            total += power(&grid, corner).unwrap();
        }
    }

    total
}

pub fn part3(notes: &str) -> u32 {
    let mut grid = Grid::parse(notes);
    let mut todo = Vec::new();
    let mut previous = 0;
    let mut total = 0;

    for y in (0..grid.height - 2).step_by(6) {
        for x in (0..grid.width - 2).step_by(6) {
            todo.push(Point::new(x, y));
        }
    }

    while previous != todo.len() {
        previous = todo.len();

        todo.retain(|&corner| {
            solve_both(&mut grid, corner);
            solve_pair(&mut grid, corner);

            if let Some(power) = power(&grid, corner) {
                total += power;
                false
            } else {
                true
            }
        });
    }

    total
}

fn solve_both(grid: &mut Grid<u8>, corner: Point) {
    for y in 2..6 {
        for x in 2..6 {
            let mut row: u32 = 0;
            let mut col: u32 = 0;

            for i in 0..8 {
                let point = corner + Point::new(i, y);
                if grid[point].is_ascii_uppercase() {
                    row |= 1 << (grid[point] - b'A');
                }

                let point = corner + Point::new(x, i);
                if grid[point].is_ascii_uppercase() {
                    col |= 1 << (grid[point] - b'A');
                }
            }

            let both = row & col;
            if both != 0 {
                let ascii = b'A' + both.trailing_zeros() as u8;
                let point = corner + Point::new(x, y);
                grid[point] = ascii;
            }
        }
    }
}

fn solve_pair(grid: &mut Grid<u8>, corner: Point) {
    for y in 2..6 {
        for x in 2..6 {
            let point = corner + Point::new(x, y);
            if grid[point] != b'.' {
                continue;
            }

            let mut freq = [0; 26];

            for i in 0..8 {
                let point = corner + Point::new(i, y);
                if grid[point].is_ascii_uppercase() {
                    freq[(grid[point] - b'A') as usize] += 1;
                }

                let point = corner + Point::new(x, i);
                if grid[point].is_ascii_uppercase() {
                    freq[(grid[point] - b'A') as usize] += 1;
                }
            }

            let Some(first) = freq.iter().position(|&f| f == 1) else { continue };
            let Some(last) = freq.iter().rposition(|&f| f == 1) else { continue };

            if first != last {
                continue;
            }

            let letter = b'A' + first as u8;
            let point = corner + Point::new(x, y);
            grid[point] = letter;

            for i in 0..8 {
                let point = corner + Point::new(i, y);
                if grid[point] == b'?' {
                    grid[point] = letter;
                }

                let point = corner + Point::new(x, i);
                if grid[point] == b'?' {
                    grid[point] = letter;
                }
            }
        }
    }
}

fn power(grid: &Grid<u8>, corner: Point) -> Option<u32> {
    let mut index = 0;
    let mut total = 0;

    for y in 2..6 {
        for x in 2..6 {
            let point = corner + Point::new(x, y);
            if !grid[point].is_ascii_uppercase() {
                return None;
            }
            index += 1;
            total += index * (grid[point] - b'A' + 1) as u32;
        }
    }

    Some(total)
}
