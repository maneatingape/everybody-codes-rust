use std::collections::VecDeque;

const SHOTS: [u8; 3] = *b"RGB";

pub fn part1(notes: &str) -> usize {
    let balloons = notes.trim().as_bytes();
    let mut index = 0;
    let mut bolts = 0;

    while index < balloons.len() {
        index += 1 + balloons[index..]
            .iter()
            .position(|&b| b != SHOTS[bolts % 3])
            .unwrap_or(balloons.len());
        bolts += 1;
    }

    bolts
}

pub fn part2(notes: &str) -> usize {
    circle(notes, 100)
}

pub fn part3(notes: &str) -> usize {
    circle(notes, 100_000)
}

fn circle(notes: &str, repeat: usize) -> usize {
    let mut left = VecDeque::from(notes.trim().as_bytes().repeat(repeat / 2));
    let mut right = left.clone();
    let mut bolts = 0;

    while let Some(balloon) = left.pop_front() {
        if left.len() < right.len() {
            let opposite = right.pop_front().unwrap();
            if balloon != SHOTS[bolts % 3] {
                left.push_back(opposite);
            }
        }
        bolts += 1;
    }

    bolts
}
