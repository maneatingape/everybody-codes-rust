use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;
use std::collections::HashSet;

pub fn part1(notes: &str) -> u64 {
    size(parse(notes))
}

pub fn part2(notes: &str) -> u64 {
    size(parse(notes))
}

pub fn part3(notes: &str) -> u64 {
    let mut constellations: Vec<Vec<Point>> = Vec::new();

    for star in parse(notes) {
        let (near, far): (Vec<_>, Vec<_>) = constellations
            .into_iter()
            .partition(|c| c.iter().any(|&next| next.manhattan(star) < 6));

        let mut merged = Vec::from([star]);
        merged.extend(near.iter().flatten());

        constellations = far;
        constellations.push(merged);
    }

    let mut sizes: Vec<_> = constellations.into_iter().map(size).collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

fn parse(notes: &str) -> Vec<Point> {
    let grid = Grid::parse(notes);
    let mut stars = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'*' {
                stars.push(point);
            }
        }
    }

    stars
}

fn size(stars: Vec<Point>) -> u64 {
    let mut heap = MinHeap::from([(0, stars[0])]);
    let mut todo: HashSet<_> = stars.into_iter().collect();
    let mut total = 0;

    while !todo.is_empty() {
        let (distance, next) = heap.pop().unwrap();

        if todo.remove(&next) {
            total += 1 + distance;
            for &star in &todo {
                heap.push(next.manhattan(star), star);
            }
        }
    }

    total as u64
}
