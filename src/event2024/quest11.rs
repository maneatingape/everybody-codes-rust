use std::collections::HashMap;

pub fn part1(notes: &str) -> u64 {
    generations(&parse(notes), "A", 4)
}

pub fn part2(notes: &str) -> u64 {
    generations(&parse(notes), "Z", 10)
}

pub fn part3(notes: &str) -> u64 {
    let kind = parse(notes);
    let population: Vec<_> = kind.keys().map(|start| generations(&kind, start, 20)).collect();
    population.iter().max().unwrap() - population.iter().min().unwrap()
}

fn parse(notes: &str) -> HashMap<&str, Vec<&str>> {
    let mut kind = HashMap::new();

    for line in notes.lines() {
        let (prefix, suffix) = line.split_once(':').unwrap();
        let children: Vec<_> = suffix.split(',').collect();
        kind.insert(prefix, children);
    }

    kind
}

fn generations(kind: &HashMap<&str, Vec<&str>>, start: &str, days: usize) -> u64 {
    let mut current = HashMap::from([(start, 1)]);
    let mut next = HashMap::new();

    for _ in 0..days {
        for (parent, population) in current.drain() {
            for &child in &kind[parent] {
                *next.entry(child).or_insert(0) += population;
            }
        }
        (current, next) = (next, current);
    }

    current.values().sum()
}
