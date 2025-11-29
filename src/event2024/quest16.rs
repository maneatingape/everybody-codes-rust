use crate::util::math::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> String {
    let machine = Machine::from(notes);
    let mut result = Vec::new();

    for (number, symbols) in machine.numbers.iter().zip(machine.symbols.iter()) {
        let top = (number * 100) % symbols.len();
        result.push(symbols[top]);
    }

    result.join(" ")
}

pub fn part2(notes: &str) -> usize {
    let machine = Machine::from(notes);

    // Wheels are all co-prime with their respective numbers,
    // so the LCM is just the LCM of the wheel sizes.
    let lcm = machine.symbols.iter().fold(1, |acc, s| acc.lcm(s.len()));
    let quotient = 202420242024 / lcm;
    let remainder = 202420242024 % lcm;

    let coins = |spins| machine.score(spins + 1, 0, 0);
    let first: usize = (0..remainder).map(coins).sum();
    let second: usize = (remainder..lcm).map(coins).sum();

    first + quotient * (first + second)
}

pub fn part3(notes: &str) -> String {
    let machine = Machine::from(notes);

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
            let score = machine.score(spins, left, right);

            next_max[i] = score + current_max[i - 1..=i + 1].iter().max().unwrap();
            next_min[i] = score + current_min[i - 1..=i + 1].iter().min().unwrap();
        }

        (current_max, next_max) = (next_max, current_max);
        (current_min, next_min) = (next_min, current_min);
    }

    format!("{} {}", current_max.iter().max().unwrap(), current_min.iter().min().unwrap())
}

struct Machine<'a> {
    numbers: Vec<usize>,
    symbols: Vec<Vec<&'a str>>,
}

impl Machine<'_> {
    fn from(notes: &str) -> Machine<'_> {
        let (prefix, suffix) = notes.split_once("\n\n").unwrap();
        let numbers: Vec<_> = prefix.iter_unsigned().collect();
        let mut symbols: Vec<_> = vec![vec![]; numbers.len()];

        for line in suffix.lines() {
            for i in (0..line.len()).step_by(4) {
                let face = &line[i..i + 3];
                if face != "   " {
                    symbols[i / 4].push(face);
                }
            }
        }

        Machine { numbers, symbols }
    }

    fn score(&self, spins: usize, left: usize, right: usize) -> usize {
        let mut freq = [0_usize; 128];

        for (number, symbols) in self.numbers.iter().zip(self.symbols.iter()) {
            let size = symbols.len();
            let top = (number * spins + right + (size - left % size)) % size;
            let face = symbols[top].as_bytes();

            freq[face[0] as usize] += 1;
            freq[face[2] as usize] += 1;
        }

        freq.iter().map(|f| f.saturating_sub(2)).sum()
    }
}
