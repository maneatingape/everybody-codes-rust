use crate::util::parse::*;
use std::cmp::Reverse;
use std::collections::HashSet;

pub fn part1(notes: &str) -> u32 {
    let mut ducks: Vec<_> = notes.lines().map(Duck::from).collect();

    while !ducks[0].has_parents(&ducks[1], &ducks[2]) {
        ducks.rotate_left(1);
    }

    ducks[0].same(&ducks[1]) * ducks[0].same(&ducks[2])
}

pub fn part2(notes: &str) -> u32 {
    links(notes).iter().map(|[child, first, second]| child.same(first) * child.same(second)).sum()
}

pub fn part3(notes: &str) -> u32 {
    let mut families: Vec<HashSet<_>> = Vec::new();

    for group in links(notes) {
        let family = families
            .extract_if(.., |family| group.iter().any(|duck| family.contains(&duck.scales)))
            .flatten()
            .chain(group.iter().map(|duck| duck.scales))
            .collect();
        families.push(family);
    }

    families.into_iter().max_by_key(HashSet::len).unwrap().iter().sum()
}

fn links(notes: &str) -> Vec<[Duck; 3]> {
    let ducks: Vec<_> = notes.lines().map(Duck::from).collect();
    let mut order = ducks.clone();
    let mut links = Vec::new();

    'outer: for child in ducks {
        order.sort_unstable_by_key(|duck| Reverse(child.same(duck)));

        for (i, first) in order.iter().enumerate().skip(2) {
            for second in &order[1..i] {
                if child.same(first) + child.same(second) < child.size {
                    continue 'outer;
                }

                if child.has_parents(first, second) {
                    links.push([child, *first, *second]);
                    continue 'outer;
                }
            }
        }
    }

    links
}

#[derive(Clone, Copy)]
struct Duck {
    scales: u32,
    size: u32,
    lo: u128,
    hi: u128,
}

impl Duck {
    fn from(line: &str) -> Self {
        let (prefix, suffix) = line.split_once(':').unwrap();

        let scales = prefix.unsigned();
        let size = suffix.len() as u32;
        let lo = suffix.bytes().fold(0, |lo, b| (lo << 1) | u128::from(b == b'G' || b == b'T'));
        let hi = suffix.bytes().fold(0, |hi, b| (hi << 1) | u128::from(b == b'C' || b == b'T'));

        Duck { scales, size, lo, hi }
    }

    fn same(&self, other: &Duck) -> u32 {
        self.size - ((self.lo ^ other.lo) | (self.hi ^ other.hi)).count_ones()
    }

    fn has_parents(&self, first: &Duck, second: &Duck) -> bool {
        let lo = (self.lo ^ first.lo) & (self.lo ^ second.lo);
        let hi = (self.hi ^ first.hi) & (self.hi ^ second.hi);
        (lo | hi) == 0
    }
}
