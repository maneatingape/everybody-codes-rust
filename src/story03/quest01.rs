use crate::util::parse::*;
use std::array::from_fn;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn part1(notes: &str) -> u32 {
    notes
        .lines()
        .filter_map(|line| {
            let (id, [red, green, blue]) = parse(line);
            (green > red.max(blue)).then_some(id)
        })
        .sum()
}

pub fn part2(notes: &str) -> u32 {
    let (.., id) = notes
        .lines()
        .map(|line| {
            let (id, [red, green, blue, shine]) = parse(line);
            (shine, Reverse(red + green + blue), id)
        })
        .max()
        .unwrap();
    id
}

pub fn part3(notes: &str) -> u32 {
    let mut groups: HashMap<_, Vec<_>> = HashMap::new();

    for (id, [red, green, blue, shine]) in notes.lines().map(parse) {
        let color = match id {
            _ if red > green.max(blue) => "red",
            _ if green > red.max(blue) => "green",
            _ if blue > red.max(green) => "blue",
            _ => continue,
        };
        let finish = match shine {
            ..=30 => "matte",
            33.. => "shiny",
            _ => continue,
        };
        groups.entry((color, finish)).or_default().push(id);
    }

    groups.values().max_by_key(|group| group.len()).unwrap().iter().sum()
}

fn parse<const N: usize>(line: &str) -> (u32, [u8; N]) {
    let id = line.unsigned();
    let mut iter = line.split_ascii_whitespace().map(binary);
    (id, from_fn(|_| iter.next().unwrap()))
}

fn binary(component: &str) -> u8 {
    component.bytes().fold(0, |acc, b| (acc << 1) | u8::from(b.is_ascii_uppercase()))
}
