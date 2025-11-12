use std::collections::HashMap;

type Rules = HashMap<u8, Vec<u8>>;

pub fn part1(notes: &str) -> &str {
    let (names, rules) = parse(notes);
    names.iter().find(|name| valid(&rules, name)).unwrap()
}

pub fn part2(notes: &str) -> u32 {
    let (names, rules) = parse(notes);
    names.iter().zip(1..).filter_map(|(name, i)| valid(&rules, name).then_some(i)).sum()
}

pub fn part3(notes: &str) -> u32 {
    let (names, rules) = parse(notes);
    let valid: Vec<_> = names
        .iter()
        .copied()
        .filter(|&name| {
            valid(&rules, name)
                && names.iter().all(|&other| name == other || !name.starts_with(other))
        })
        .collect();

    let mut current = HashMap::new();
    let mut next = HashMap::new();
    let mut result = 0;

    for size in 1..=11 {
        for &name in &valid {
            if name.len() == size {
                let last = *name.as_bytes().last().unwrap();
                *current.entry(last).or_insert(0) += 1;
            }
        }

        if size >= 7 {
            result += current.values().sum::<u32>();
        }

        for (from, amount) in current.drain() {
            for &to in &rules[&from] {
                *next.entry(to).or_insert(0) += amount;
            }
        }

        (current, next) = (next, current);
    }

    result
}

fn parse(notes: &str) -> (Vec<&str>, Rules) {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();

    let names = prefix.split(',').collect();
    let rules = suffix
        .lines()
        .map(|line| {
            let key = line.bytes().next().unwrap();
            let value = line.bytes().skip(4).step_by(2).collect();
            (key, value)
        })
        .collect();

    (names, rules)
}

fn valid(rules: &Rules, name: &str) -> bool {
    name.as_bytes().windows(2).all(|w| rules[&w[0]].contains(&w[1]))
}
