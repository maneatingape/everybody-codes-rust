use crate::util::parse::*;
use std::array::from_fn;
use std::collections::{HashMap, HashSet};

type Dance = [Vec<usize>; 4];

pub fn part1(notes: &str) -> usize {
    let mut dance = parse(notes);

    for round in 0..10 {
        shuffle(&mut dance, round);
    }

    shout(&dance)
}

pub fn part2(notes: &str) -> usize {
    let mut dance = parse(notes);
    let mut seen = HashMap::new();
    let mut round = 0;

    loop {
        shuffle(&mut dance, round);
        round += 1;

        let count = seen.entry(key(&dance)).or_insert(0);
        *count += 1;

        if *count == 2024 {
            break round * shout(&dance);
        }
    }
}

pub fn part3(notes: &str) -> usize {
    let mut dance = parse(notes);
    let mut seen = HashSet::new();
    let mut round = 0;

    while seen.insert(dance.clone()) {
        shuffle(&mut dance, round);
        round += 1;
    }

    let highest = seen.iter().max_by_key(|dance| key(dance)).unwrap();
    shout(highest)
}

fn parse(notes: &str) -> Dance {
    let mut dance = from_fn(|_| Vec::new());

    for (i, n) in notes.iter_unsigned().enumerate() {
        dance[i % 4].push(n);
    }

    dance
}

fn shuffle(dance: &mut Dance, round: usize) {
    let clapper = dance[round % 4].remove(0);
    let dest = (round + 1) % 4;
    let size = 2 * dance[dest].len();
    let remainder = (clapper - 1) % size;
    let offset = remainder.min(size - remainder);

    dance[dest].insert(offset, clapper);
}

fn key(dance: &Dance) -> usize {
    (dance[0][0] << 48) | (dance[1][0] << 32) | (dance[2][0] << 16) | dance[3][0]
}

fn shout(dance: &Dance) -> usize {
    format!("{}{}{}{}", dance[0][0], dance[1][0], dance[2][0], dance[3][0]).unsigned()
}
