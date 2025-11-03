use crate::util::parse::*;

pub fn part1(notes: &str) -> &str {
    let (names, moves, size) = parse(notes);
    let index = moves.iter().fold(0, |index, amount| (index + amount).clamp(0, size - 1));
    names[index as usize]
}

pub fn part2(notes: &str) -> &str {
    let (names, moves, size) = parse(notes);
    let index = moves.iter().sum::<i32>().rem_euclid(size);
    names[index as usize]
}

pub fn part3(notes: &str) -> &str {
    let (mut names, moves, size) = parse(notes);
    moves.iter().for_each(|amount| names.swap(0, amount.rem_euclid(size) as usize));
    names[0]
}

fn parse(notes: &str) -> (Vec<&str>, Vec<i32>, i32) {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();

    let names: Vec<_> = prefix.split(',').collect();
    let size = names.len() as i32;

    let direction = suffix.bytes().filter(u8::is_ascii_uppercase);
    let amount = suffix.iter_signed::<i32>();
    let moves = direction.zip(amount).map(|(d, a)| if d == b'R' { a } else { -a }).collect();

    (names, moves, size)
}
