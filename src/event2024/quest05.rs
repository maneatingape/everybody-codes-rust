use std::array::from_fn;
use std::collections::{HashMap, HashSet};

use crate::util::parse::*;

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

        let number = shout(&dance);
        let count = seen.entry(number).or_insert(0);
        *count += 1;

        if *count == 2024 {
            break round * number;
        }
    }
}

pub fn part3(notes: &str) -> usize {
    let mut dance = parse(notes);
    let mut seen = HashSet::new();
    let mut round = 0;
    let mut highest = 0;

    while seen.insert((dance.clone(), round % 4)) {
        shuffle(&mut dance, round);
        round += 1;
        highest = highest.max(shout(&dance));
    }

    highest
}

fn parse(notes: &str) -> Dance {
    let numbers: Vec<_> = notes.iter_unsigned().collect();
    from_fn(|column| numbers.iter().skip(column).step_by(4).copied().collect())
}

fn shuffle(dance: &mut Dance, round: usize) {
    let clapper = dance[round % 4].remove(0);
    let dest = (round + 1) % 4;
    let size = 2 * dance[dest].len();
    let remainder = (clapper - 1) % size;
    let offset = remainder.min(size - remainder);

    dance[dest].insert(offset, clapper);
}

fn shout(dance: &Dance) -> usize {
    format!("{}{}{}{}", dance[0][0], dance[1][0], dance[2][0], dance[3][0]).unsigned()
}
