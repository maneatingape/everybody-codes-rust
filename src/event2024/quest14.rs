use crate::util::parse::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Segment = (i32, i32, i32);

pub fn part1(notes: &str) -> i32 {
    let (segments, _) = grow_plant(notes);
    segments.iter().map(|p| p.1).max().unwrap()
}

pub fn part2(notes: &str) -> usize {
    let (segments, _) = grow_plant(notes);
    segments.len()
}

pub fn part3(notes: &str) -> i32 {
    let (segments, leaves) = grow_plant(notes);
    let mut murkiness = HashMap::new();

    for leaf in leaves {
        let mut todo = VecDeque::from([(leaf, 0)]);
        let mut seen = HashSet::from([leaf]);

        while let Some((segment @ (x, y, z), distance)) = todo.pop_front() {
            let orthogonal = [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ];

            if x == 0 && z == 0 {
                *murkiness.entry(segment).or_insert(0) += distance;
            }

            for next in orthogonal {
                if segments.contains(&next) && seen.insert(next) {
                    todo.push_back((next, distance + 1));
                }
            }
        }
    }

    *murkiness.values().min().unwrap()
}

fn grow_plant(notes: &str) -> (HashSet<Segment>, HashSet<Segment>) {
    let mut segments = HashSet::new();
    let mut leaves = HashSet::new();

    for line in notes.lines() {
        let directions = line.bytes().filter(u8::is_ascii_uppercase);
        let numbers = line.iter_unsigned::<usize>();

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for (direction, number) in directions.zip(numbers) {
            let (dx, dy, dz) = match direction {
                b'U' => (0, 1, 0),
                b'D' => (0, -1, 0),
                b'R' => (1, 0, 0),
                b'L' => (-1, 0, 0),
                b'F' => (0, 0, 1),
                b'B' => (0, 0, -1),
                _ => unreachable!(),
            };
            for _ in 0..number {
                x += dx;
                y += dy;
                z += dz;
                segments.insert((x, y, z));
            }
        }

        leaves.insert((x, y, z));
    }

    (segments, leaves)
}
