use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> u32 {
    let (words, inscription) = parse(notes, false);
    let mut runes = 0;

    for word in &words {
        for i in 0..inscription.len() {
            if inscription[i..].starts_with(word) {
                runes += 1;
            }
        }
    }

    runes
}

pub fn part2(notes: &str) -> u32 {
    let (words, inscription) = parse(notes, true);
    let mut runes = vec![0; inscription.len()];

    for word in &words {
        for i in 0..inscription.len() {
            if inscription[i..].starts_with(word) {
                for j in 0..word.len() {
                    runes[i + j] = 1;
                }
            }
        }
    }

    runes.iter().sum()
}

pub fn part3(notes: &str) -> u32 {
    let (words, scales) = parse(notes, true);
    let grid = Grid::parse(scales);
    let mut runes = grid.default_copy();

    for word in &words {
        for x in 0..grid.width {
            for y in 0..grid.height {
                let start = Point::new(x, y);
                find(word, &grid, &mut runes, start, DOWN);
                find(word, &grid, &mut runes, start, RIGHT);
            }
        }
    }

    runes.bytes.iter().sum()
}

fn parse(notes: &str, reverse: bool) -> (Vec<String>, &str) {
    let (first, second) = notes.split_once("\n\n").unwrap();
    let mut words = Vec::new();

    for token in first[6..].split(',') {
        words.push(String::from(token));
        if reverse {
            words.push(token.chars().rev().collect());
        }
    }

    (words, second)
}

fn find(word: &str, grid: &Grid<u8>, found: &mut Grid<u32>, start: Point, direction: Point) {
    let mut position = start;

    for b in word.bytes() {
        if !grid.contains(position) || grid[position] != b {
            return;
        }
        position += direction;
        position.x = position.x.rem_euclid(grid.width);
    }

    let mut position = start;

    for _ in 0..word.len() {
        found[position] = 1;
        position += direction;
        position.x = position.x.rem_euclid(grid.width);
    }
}
