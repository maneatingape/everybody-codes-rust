use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    let numbers: Vec<_> = notes.iter_unsigned::<u64>().collect();
    let mut wheel = vec![1];

    wheel.extend(numbers.iter().step_by(2));
    wheel.extend(numbers.iter().skip(1).step_by(2).rev());

    wheel[2025 % wheel.len()]
}

pub fn part2(notes: &str) -> u64 {
    ranges(notes, 20252025)
}

pub fn part3(notes: &str) -> u64 {
    ranges(notes, 202520252025)
}

fn ranges(notes: &str, turns: u64) -> u64 {
    let pairs: Vec<_> = notes.iter_unsigned::<u64>().chunk::<2>().collect();
    let mut wheel = vec![(1, 1, 1)];

    for &[start, end] in pairs.iter().step_by(2) {
        wheel.push((start, end, end - start + 1));
    }

    for &[start, end] in pairs.iter().skip(1).step_by(2).rev() {
        wheel.push((end, start, end - start + 1));
    }

    let total: u64 = wheel.iter().map(|(_, _, size)| size).sum();
    let mut remaining = turns % total;

    for (start, end, size) in wheel {
        if remaining >= size {
            remaining -= size;
        } else {
            return if start < end { start + remaining } else { start - remaining };
        }
    }

    unreachable!()
}
