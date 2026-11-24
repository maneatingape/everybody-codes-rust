use crate::util::math::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> String {
    parse(notes)
        .iter()
        .map(|(number, faces)| faces[number * 100 % faces.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn part2(notes: &str) -> usize {
    let wheels = parse(notes);

    // Wheels are all coprime with their respective numbers,
    // so the LCM is just the LCM of the wheel sizes.
    let lcm = wheels.iter().fold(1, |acc, (_, faces)| acc.lcm(faces.len()));
    let quotient = 202420242024 / lcm;
    let remainder = 202420242024 % lcm;

    let coins = |spins| score(&wheels, spins + 1, 0, 0);
    let first: usize = (0..remainder).map(coins).sum();
    let second: usize = (remainder..lcm).map(coins).sum();

    first + quotient * (first + second)
}

pub fn part3(notes: &str) -> String {
    let wheels = parse(notes);

    let mut current_max = [0; 515];
    let mut next_max = [0; 515];
    let mut current_min = [usize::MAX; 515];
    let mut next_min = [usize::MAX; 515];

    let middle = 257;
    current_min[middle] = 0;

    for spins in 1..=256 {
        for i in (middle - spins)..=(middle + spins) {
            let left = middle.saturating_sub(i);
            let right = i.saturating_sub(middle);
            let coins = score(&wheels, spins, left, right);

            next_max[i] = coins + current_max[i - 1..=i + 1].iter().max().unwrap();
            next_min[i] = coins + current_min[i - 1..=i + 1].iter().min().unwrap();
        }

        (current_max, next_max) = (next_max, current_max);
        (current_min, next_min) = (next_min, current_min);
    }

    format!("{} {}", current_max.iter().max().unwrap(), current_min.iter().min().unwrap())
}

fn parse(notes: &str) -> Vec<(usize, Vec<&str>)> {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();

    prefix
        .iter_unsigned()
        .enumerate()
        .map(|(i, number)| {
            let faces = suffix
                .lines()
                .filter_map(|line| line.get(4 * i..4 * i + 3))
                .filter(|&face| face != "   ")
                .collect();
            (number, faces)
        })
        .collect()
}

fn score(wheels: &[(usize, Vec<&str>)], spins: usize, left: usize, right: usize) -> usize {
    let mut freq = [0_usize; 128];

    for (number, faces) in wheels {
        let size = faces.len();
        let top = (number * spins + right + size - left % size) % size;
        let face = faces[top].as_bytes();

        freq[usize::from(face[0])] += 1;
        freq[usize::from(face[2])] += 1;
    }

    freq.iter().map(|f| f.saturating_sub(2)).sum()
}
