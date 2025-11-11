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
    names
        .iter()
        .filter(|&name| {
            valid(&rules, name)
                && names.iter().all(|other| name == other || !name.starts_with(other))
        })
        .map(|name| recurse(&rules, *name.as_bytes().last().unwrap(), name.len()))
        .sum()
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

fn recurse(rules: &Rules, last: u8, size: usize) -> u32 {
    let this = u32::from(size >= 7);

    if size < 11 {
        this + rules[&last].iter().map(|&next| recurse(rules, next, size + 1)).sum::<u32>()
    } else {
        this
    }
}
