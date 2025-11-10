use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    let (quality, ..) = score(notes);
    quality
}

pub fn part2(notes: &str) -> u64 {
    let ratings: Vec<_> = notes.lines().map(score).map(|(quality, ..)| quality).collect();
    *ratings.iter().max().unwrap() - *ratings.iter().min().unwrap()
}

pub fn part3(notes: &str) -> usize {
    let mut swords: Vec<_> = notes.lines().map(score).collect();
    swords.sort_unstable();
    swords.into_iter().rev().enumerate().map(|(i, (.., id))| (i + 1) * id).sum()
}

pub fn score(line: &str) -> (u64, Vec<u64>, usize) {
    let mut levels: Vec<(Option<u64>, u64, Option<u64>)> = Vec::new();

    'outer: for n in line.iter_unsigned::<u64>().skip(1) {
        for (left, middle, right) in &mut levels {
            if left.is_none() && n < *middle {
                *left = Some(n);
                continue 'outer;
            }
            if right.is_none() && n > *middle {
                *right = Some(n);
                continue 'outer;
            }
        }

        levels.push((None, n, None));
    }

    let id = line.unsigned();
    let quality = levels.iter().map(|&(_, middle, _)| middle).fold(0, decimal);
    let levels = levels
        .into_iter()
        .map(|(left, middle, right)| {
            [left, Some(middle), right].into_iter().flatten().fold(0, decimal)
        })
        .collect();

    (quality, levels, id)
}

fn decimal(number: u64, digit: u64) -> u64 {
    10 * number + digit
}
