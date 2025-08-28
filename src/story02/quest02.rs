use std::collections::VecDeque;

const SHOTS: [u8; 3] = [b'R', b'G', b'B'];

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
    let half = notes.trim().as_bytes().repeat(repeat / 2);

    let mut left = VecDeque::from(half);
    let mut right = left.clone();
    let mut bolts = 0;

    while let Some(balloon) = left.pop_front() {
        let bolt = SHOTS[bolts % 3];
        bolts += 1;

        if left.len() < right.len() && balloon == bolt {
            right.pop_front();
        }

        if left.len() < right.len() {
            left.push_back(right.pop_front().unwrap());
        }
    }

    bolts
}
