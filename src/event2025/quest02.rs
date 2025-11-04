use crate::util::iter::*;
use crate::util::parse::*;
use std::iter::successors;
use std::ops::RangeInclusive;

const RANGE: RangeInclusive<i64> = -1_000_000..=1_000_000;

pub fn part1(notes: &str) -> String {
    let [x0, y0] = parse(notes);
    let (x, y) =
        (0..3).fold((0, 0), |(x, y), _| (x0 + (x * x - y * y) / 10, y0 + (2 * x * y) / 10));
    format!("[{x},{y}]")
}

pub fn part2(notes: &str) -> u32 {
    let [x0, y0] = parse(notes);
    (0..1001)
        .step_by(10)
        .flat_map(|x1| (0..1001).step_by(10).map(move |y1| engrave(x0 + x1, y0 + y1)))
        .sum()
}

pub fn part3(notes: &str) -> u32 {
    let [x0, y0] = parse(notes);
    (0..1001).flat_map(|x1| (0..1001).map(move |y1| engrave(x0 + x1, y0 + y1))).sum()
}

fn parse(notes: &str) -> [i64; 2] {
    notes.iter_signed::<i64>().chunk::<2>().next().unwrap()
}

fn engrave(x2: i64, y2: i64) -> u32 {
    successors(Some((0, 0)), |(x, y)| {
        Some((x2 + (x * x - y * y) / 100_000, y2 + (2 * x * y) / 100_000))
    })
    .take(101)
    .all(|(x, y)| RANGE.contains(&x) && RANGE.contains(&y)) as u32
}
