use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    let mut ducks = parse(notes);
    let mut sorted = false;

    for _ in 0..10 {
        sorted |= ducks.is_sorted();

        for i in 0..ducks.len() - 1 {
            let (from, to) = if sorted { (i + 1, i) } else { (i, i + 1) };

            if ducks[from] > ducks[to] {
                ducks[from] -= 1;
                ducks[to] += 1;
            }
        }
    }

    ducks.iter().zip(1..).map(|(duck, column)| duck * column).sum()
}

pub fn part2(notes: &str) -> usize {
    let mut ducks = parse(notes);

    while !ducks.is_sorted() {
        for i in 0..ducks.len() - 1 {
            if ducks[i] > ducks[i + 1] {
                let delta = (ducks[i] - ducks[i + 1]).div_ceil(2);
                ducks[i] -= delta;
                ducks[i + 1] += delta;
            }
        }
    }

    let mut flow = 0;
    let mut start = 0;
    let mut extra = 0;
    let mut phase1 = 0;

    for (&duck, initial) in ducks.iter().zip(parse(notes)) {
        if flow == 0 {
            start = duck;
            extra = 0;
        }

        flow = flow + initial - duck;
        extra += duck - start;
        phase1 = phase1.max(flow + extra);
    }

    phase1 + phase2(&ducks)
}

pub fn part3(notes: &str) -> usize {
    phase2(&parse(notes))
}

fn parse(notes: &str) -> Vec<usize> {
    notes.iter_unsigned().collect()
}

fn phase2(ducks: &[usize]) -> usize {
    let average = ducks.iter().sum::<usize>() / ducks.len();
    ducks.iter().map(|&duck| average.saturating_sub(duck)).sum()
}
