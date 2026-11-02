use std::collections::{HashMap, HashSet};

pub fn part1(notes: &str) -> usize {
    reachable(notes, "NULL", false).len() - 1
}

pub fn part2(notes: &str) -> usize {
    reachable(notes, "NULL", true).len() - 1
}

pub fn part3(notes: &str) -> usize {
    let [first, second, third] =
        ["NULLv1", "NULLv2", "NULLv3"].map(|start| reachable(notes, start, true));

    first
        .union(&second)
        .filter(|&node| third.contains(node) || (first.contains(node) && second.contains(node)))
        .count()
}

fn reachable<'a>(notes: &'a str, start: &'a str, reverse: bool) -> HashSet<&'a str> {
    let mut graph = HashMap::new();

    for line in notes.lines() {
        let (prefix, suffix) = line.split_once(':').unwrap();

        for child in suffix.split(',') {
            let (from, to) = if reverse { (child, prefix) } else { (prefix, child) };
            graph.entry(from).or_insert_with(Vec::new).push(to);
        }
    }

    let mut todo = vec![start];
    let mut seen = HashSet::from([start]);

    while let Some(node) = todo.pop() {
        todo.extend(graph.get(node).into_iter().flatten().filter(|&&next| seen.insert(next)));
    }

    seen
}
