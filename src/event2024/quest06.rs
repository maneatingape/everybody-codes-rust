use std::collections::HashMap;

pub fn part1(notes: &str) -> String {
    solve(notes, false)
}

pub fn part2(notes: &str) -> String {
    solve(notes, true)
}

pub fn part3(notes: &str) -> String {
    solve(notes, true)
}

fn solve(notes: &str, first_letter_only: bool) -> String {
    let mut apples = Vec::new();
    let mut parents = HashMap::new();
    let mut branches = HashMap::new();

    for line in notes.lines() {
        let (parent, children) = line.split_once(':').unwrap();

        for child in children.split(',') {
            if child == "@" {
                apples.push(parent);
            } else {
                parents.insert(child, parent);
            }
        }
    }

    'outer: for apple in apples {
        let mut current = apple;
        let mut path = vec!["@", apple];

        while let Some(&next) = parents.get(current) {
            if path.contains(&next) {
                continue 'outer;
            }
            current = next;
            path.push(next);
        }

        branches.entry(path.len()).or_insert_with(Vec::new).push(path);
    }

    let powerful = branches.values().find(|p| p.len() == 1).unwrap().first().unwrap();
    powerful.iter().rev().map(|p| if first_letter_only { &p[..1] } else { p }).collect()
}
