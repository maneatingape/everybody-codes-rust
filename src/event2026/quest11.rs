use std::collections::VecDeque;

use crate::util::iter::*;
use crate::util::parse::*;

type Ring = Vec<Vec<usize>>;

pub fn part1(notes: &str) -> u32 {
    let (ring, normal) = parse(notes, 1);
    connect(&ring, normal, &[])
}

pub fn part2(notes: &str) -> u32 {
    let (ring, normal) = parse(notes, 3);
    let size = ring.len();

    (0..size)
        .flat_map(|from| (from + 1..size).map(move |to| [from, to]))
        .map(|pair| connect(&ring, normal, &[pair]))
        .max()
        .unwrap()
}

pub fn part3(notes: &str) -> u32 {
    let (ring, normal) = parse(notes, 3);
    let interval = ring.len() / 8;

    let mut links = Vec::new();
    permute([0, 1, 2, 3, 4, 5, 6, 7], &mut links, 0);

    (0..interval)
        .flat_map(|base| {
            links.iter().map(move |pairs| pairs.map(|pair| pair.map(|i| base + interval * i)))
        })
        .map(|pairs| connect(&ring, normal, &pairs))
        .max()
        .unwrap()
}

fn parse(notes: &str, neighbours: usize) -> (Ring, u32) {
    let size = notes.unsigned();
    let mut ring = (0..size)
        .map(|from| {
            (1..=neighbours)
                .chain(size - neighbours..size)
                .map(|delta| (from + delta) % size)
                .collect()
        })
        .collect();

    let normal = strain(&ring, 0);

    for [from, to] in notes.iter_unsigned::<usize>().skip(1).chunk::<2>() {
        ring[from - 1].push(to - 1);
        ring[to - 1].push(from - 1);
    }

    (ring, normal)
}

fn connect(ring: &Ring, normal: u32, pairs: &[[usize; 2]]) -> u32 {
    let mut ring = ring.clone();

    for &[from, to] in pairs {
        ring[from].push(to);
        ring[to].push(from);
    }

    (0..ring.len()).map(|start| normal - strain(&ring, start)).sum()
}

fn strain(ring: &Ring, start: usize) -> u32 {
    let mut todo = VecDeque::from([(start, 0)]);
    let mut seen = vec![false; ring.len()];
    let mut sum = 0;

    seen[start] = true;

    while let Some((from, cost)) = todo.pop_front() {
        sum += cost;

        for &to in &ring[from] {
            if !seen[to] {
                todo.push_back((to, cost + 1));
                seen[to] = true;
            }
        }
    }

    sum
}

fn permute(mut array: [usize; 8], links: &mut Vec<[[usize; 2]; 4]>, i: usize) {
    if i == 8 {
        links.push(array.as_chunks::<2>().0.try_into().unwrap());
    } else {
        for j in i + 1..8 {
            array.swap(i + 1, j);
            permute(array, links, i + 2);
            array.swap(i + 1, j);
        }
    }
}
