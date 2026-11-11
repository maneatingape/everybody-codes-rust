use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> i64 {
    solve(notes)
}

pub fn part2(notes: &str) -> i64 {
    notes.split("\n\n").map(solve).sum()
}

pub fn part3(notes: &str) -> i64 {
    notes.split("\n\n").map(solve).sum()
}

fn solve(clock: &str) -> i64 {
    let [marks, current, limit] =
        clock.iter_signed::<i64>().chunk::<3>().chunk::<3>().next().unwrap();
    let directions = clock.split_ascii_whitespace().nth(3).unwrap().split(':');

    let lcm = marks[0].lcm(marks[1]).lcm(marks[2]);
    let mut hands: Vec<_> = directions
        .enumerate()
        .map(|(i, direction)| {
            let scale = lcm / marks[i];
            let speed = if direction == "CW" { scale } else { -scale };
            (current[i] * scale, speed, limit[i])
        })
        .collect();

    hands.sort_by_key(|&(.., limit)| limit);

    [(0, 1), (0, 2), (1, 2)]
        .into_iter()
        .map(|(i, j)| {
            let (p1, v1, s1) = hands[i];
            let (p2, v2, s2) = hands[j];

            let first = p2 - p1;
            let second = first + (v2 - v1) * s1;
            let third = second + v2 * (s2 - s1);

            meetings(first, second, lcm) + meetings(second, third, lcm)
        })
        .sum()
}

fn meetings(from: i64, to: i64, lcm: i64) -> i64 {
    let sign = (to - from).signum();
    (sign * to).div_euclid(lcm) - (sign * from).div_euclid(lcm)
}
