use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let center = grid.width / 2;
    let volcano = Point::new(center, center);

    let result: i32 = grid
        .points()
        .filter_map(|point| (radius(point, volcano) <= 10).then_some(to_decimal(grid[point])))
        .sum();

    result - to_decimal(grid[volcano])
}

pub fn part2(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let center = grid.width / 2;
    let volcano = Point::new(center, center);

    let mut rings = vec![0; 2 * center as usize];

    for point in grid.points() {
        rings[radius(point, volcano) as usize] += to_decimal(grid[point]);
    }

    rings.iter().zip(0..).max_by_key(|&(v, _)| v).map(|(v, r)| v * r).unwrap()
}

pub fn part3(notes: &str) -> i32 {
    let grid = Grid::parse(notes);
    let center = grid.width / 2;
    let volcano = Point::new(center, center);
    let end = grid.find(b'S').unwrap();

    let mut split = grid.clone();
    split[end] = b'0';

    (0..center)
        .find_map(|r| {
            for point in grid.points() {
                if radius(point, volcano) <= r || (point.x == center && point.y > center) {
                    split[point] = b'#';
                }
            }

            let start = Point::new(center, center + r + 1);
            let total = to_decimal(grid[start])
                + dijkstra(&split, start + LEFT, end)
                + dijkstra(&split, start + RIGHT, end);

            let limit = 30 * (r + 1);
            (total < limit).then_some(r * total)
        })
        .unwrap()
}

fn dijkstra(grid: &Grid<u8>, start: Point, end: Point) -> i32 {
    let mut todo = MinHeap::new();
    let mut seen = grid.same_size_with(i32::MAX);

    let cost = to_decimal(grid[start]);
    todo.push(cost, start);
    seen[start] = cost;

    while let Some((cost, point)) = todo.pop() {
        if point == end {
            return cost;
        }

        for next in ORTHOGONAL.map(|o| o + point) {
            if grid.contains(next) && grid[next] != b'#' {
                let next_cost = cost + to_decimal(grid[next]);

                if seen[next] > next_cost {
                    todo.push(next_cost, next);
                    seen[next] = next_cost;
                }
            }
        }
    }

    unreachable!()
}

fn to_decimal(b: u8) -> i32 {
    (b - b'0') as i32
}

fn radius(point: Point, volcano: Point) -> i32 {
    let delta = point - volcano;
    let squared = delta.x * delta.x + delta.y * delta.y;
    let radius = squared.isqrt();

    if radius * radius < squared { radius + 1 } else { radius }
}
