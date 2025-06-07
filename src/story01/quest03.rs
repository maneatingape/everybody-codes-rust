use crate::util::iter::*;
use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    parse(notes)
        .iter()
        .map(|[x, y]| {
            let size = x + y - 1;
            let nx = (x - 1 + 100) % size;
            (nx + 1) + 100 * (size - nx)
        })
        .sum()
}

pub fn part2(notes: &str) -> u64 {
    search(notes)
}

pub fn part3(notes: &str) -> u64 {
    search(notes)
}

fn parse(notes: &str) -> Vec<[u64; 2]> {
    notes.iter_unsigned::<u64>().chunk::<2>().collect()
}

fn search(notes: &str) -> u64 {
    let mut time = 0;
    let mut factor = 1;

    for [x, y] in parse(notes) {
        let size = x + y - 1;

        while (x + time) % size != 0 {
            time += factor;
        }

        factor *= size;
    }

    time
}
