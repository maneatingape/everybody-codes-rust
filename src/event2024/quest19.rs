use crate::util::grid::*;
use crate::util::point::*;

const NEIGHBOURS: [Point; 8] = [
    Point::new(-1, -1),
    Point::new(0, -1),
    Point::new(1, -1),
    Point::new(1, 0),
    Point::new(1, 1),
    Point::new(0, 1),
    Point::new(-1, 1),
    Point::new(-1, 0),
];

pub fn part1(notes: &str) -> String {
    decode(notes, 1)
}

pub fn part2(notes: &str) -> String {
    decode(notes, 100)
}

pub fn part3(notes: &str) -> String {
    decode(notes, 1048576000)
}

fn decode(notes: &str, rounds: u32) -> String {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let mut operations = prefix.bytes().cycle();
    let mut grid @ Grid { width, height, .. } = Grid::parse(suffix);

    // Map points to points
    let mut lookup = Grid {
        width,
        height,
        bytes: (0..width * height).map(|i| Point::new(i % width, i / width)).collect(),
    };

    // Apply 1 round of unscrambling
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let point = Point::new(x, y);
            let mut cells = NEIGHBOURS.map(|n| lookup[point + n]);

            if operations.next().unwrap() == b'R' {
                cells.rotate_right(1);
            } else {
                cells.rotate_left(1);
            }

            NEIGHBOURS.iter().zip(cells).for_each(|(&n, c)| lookup[point + n] = c);
        }
    }

    // Exponentiation by squaring
    let mut exponent = 1;

    while exponent <= rounds {
        if exponent & rounds != 0 {
            grid = unscramble(&grid, &lookup);
        }
        lookup = unscramble(&lookup, &lookup);
        exponent *= 2;
    }

    // Extract message
    let left = grid.find(b'>').unwrap();
    let right = grid.find(b'<').unwrap();

    (left.x + 1..right.x).map(|x| grid[Point::new(x, left.y)] as char).collect()
}

fn unscramble<T: Copy>(grid: &Grid<T>, lookup: &Grid<Point>) -> Grid<T> {
    let mut next = grid.clone();

    for point in grid.points() {
        next[point] = grid[lookup[point]];
    }

    next
}
