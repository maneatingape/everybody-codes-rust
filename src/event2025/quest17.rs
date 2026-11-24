use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> i32 {
    let (grid, volcano) = parse(notes);
    grid.points()
        .filter(|&point| point != volcano && radius(point, volcano) <= 10)
        .map(|point| grid[point].to_decimal::<i32>())
        .sum()
}

pub fn part2(notes: &str) -> i32 {
    let (grid, volcano) = parse(notes);
    let mut rings = vec![0; 2 * volcano.x as usize];

    for point in grid.points() {
        rings[radius(point, volcano) as usize] += grid[point].to_decimal::<i32>();
    }

    rings.iter().zip(0..).max().map(|(d, r)| d * r).unwrap()
}

pub fn part3(notes: &str) -> i32 {
    let (grid, volcano) = parse(notes);
    let start = grid.find(b'S').unwrap();
    let closest = radius(start, volcano);

    let mut todo = MinHeap::from([(0, (start, 0, closest))]);
    let mut seen = grid.same_size_with([0; 2]);
    seen[start][0] = closest;

    while let Some((time, (point, parity, closest))) = todo.pop() {
        for next in ORTHOGONAL.map(|o| o + point) {
            if !grid.contains(next) {
                continue;
            }
            if next == start && parity == 1 {
                return time * (time / 30);
            }

            let crossed = point.y > volcano.y && (point.x < volcano.x) != (next.x < volcano.x);
            let next_parity = parity ^ usize::from(crossed);
            let next_time = time + grid[next].to_decimal::<i32>();
            let next_closest = closest.min(radius(next, volcano));

            if next_closest > next_time / 30 && next_closest > seen[next][next_parity] {
                todo.push(next_time, (next, next_parity, next_closest));
                seen[next][next_parity] = next_closest;
            }
        }
    }

    unreachable!()
}

fn parse(notes: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(notes);
    let volcano = grid.find(b'@').unwrap();
    (grid, volcano)
}

fn radius(point: Point, volcano: Point) -> i32 {
    let delta = point - volcano;
    let squared = delta.x * delta.x + delta.y * delta.y;
    let radius = squared.isqrt();
    if radius * radius == squared { radius } else { radius + 1 }
}
