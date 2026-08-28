use std::collections::{HashSet, VecDeque};

use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> usize {
    let (mut swarm, beacons, moves) = parse(notes);
    let mut sky = HashSet::from([swarm]);

    for next in moves {
        swarm = midpoint(swarm, beacons[next]);
        sky.insert(swarm);
    }

    sky.len()
}

pub fn part2(notes: &str) -> usize {
    let (mut swarm, beacons, moves) = parse(notes);
    let mut sky = HashSet::from([swarm]);

    for next in moves {
        swarm = midpoint(swarm, beacons[next]);
        sky.insert(swarm);
    }

    fireflies(&sky)
}

pub fn part3(notes: &str) -> usize {
    let (swarm, beacons, _) = parse(notes);
    let mut sky = HashSet::from([swarm]);
    let mut todo = VecDeque::from([swarm]);

    while let Some(swarm) = todo.pop_front() {
        for to in beacons {
            let next = midpoint(swarm, to);
            if sky.insert(next) {
                todo.push_back(next);
            }
        }
    }

    fireflies(&sky)
}

fn parse(notes: &str) -> (Point, [Point; 3], impl Iterator<Item = usize>) {
    let points = notes.iter_signed().chunk::<2>().map(|[x, y]| Point::new(x, y));
    let [swarm, beacons @ ..] = points.chunk::<4>().next().unwrap();

    let (_, suffix) = notes.rsplit_once('=').unwrap();
    let moves = suffix.bytes().map(|b| usize::from(b - b'A'));

    (swarm, beacons, moves)
}

fn midpoint(from: Point, to: Point) -> Point {
    Point::new(from.x.midpoint(to.x), from.y.midpoint(to.y))
}

fn fireflies(sky: &HashSet<Point>) -> usize {
    sky.iter()
        .flat_map(|&point| ORTHOGONAL.map(|o| point + o))
        .collect::<HashSet<_>>()
        .difference(sky)
        .count()
}
