use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    let mut ducks: Vec<u64> = notes.iter_unsigned().collect();
    let mut rounds = 0;

    while ducks.windows(2).any(|w| w[0] > w[1]) {
        for i in 0..ducks.len() - 1 {
            if ducks[i] > ducks[i + 1] {
                ducks[i] -= 1;
                ducks[i + 1] += 1;
            }
        }
        rounds += 1;
    }

    for _ in rounds..10 {
        for i in 0..ducks.len() - 1 {
            if ducks[i] < ducks[i + 1] {
                ducks[i] += 1;
                ducks[i + 1] -= 1;
            }
        }
    }

    ducks.iter().zip(1..).map(|(d, i)| d * i).sum()
}

pub fn part2(notes: &str) -> usize {
    let ducks: Vec<usize> = notes.iter_unsigned().collect();
    let mut ascending = ducks.clone();

    while ascending.windows(2).any(|w| w[0] > w[1]) {
        for i in 0..ascending.len() - 1 {
            if ascending[i] > ascending[i + 1] {
                let delta = (ascending[i] - ascending[i + 1]).div_ceil(2);
                ascending[i] -= delta;
                ascending[i + 1] += delta;
            }
        }
    }

    let (.., phase1) =
        ducks.iter().zip(&ascending).fold((0, 0, 0), |(sum1, sum2, phase1), (first, second)| {
            (sum1 + first, sum2 + second, phase1.max(sum1 - sum2))
        });

    let average = ascending.iter().sum::<usize>() / ascending.len();
    let phase2 = ascending.iter().map(|&duck| average.saturating_sub(duck)).sum::<usize>();
    phase1 + phase2
}

pub fn part3(notes: &str) -> usize {
    let ascending: Vec<_> = notes.iter_unsigned().collect();
    let average = ascending.iter().sum::<usize>() / ascending.len();
    ascending.iter().map(|&duck| average.saturating_sub(duck)).sum()
}
