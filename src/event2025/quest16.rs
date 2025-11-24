use crate::util::parse::*;

const BLOCKS: usize = 202520252025000;

pub fn part1(notes: &str) -> usize {
    notes.iter_unsigned().map(|n: usize| 90 / n).sum()
}

pub fn part2(notes: &str) -> usize {
    spell(notes).iter().product()
}

pub fn part3(notes: &str) -> usize {
    let spell = spell(notes);
    let mut upper = BLOCKS;
    let mut lower = 0;

    while lower != upper {
        let middle = (lower + upper).div_ceil(2);

        if spell.iter().map(|n| middle / n).sum::<usize>() > BLOCKS {
            upper = middle - 1;
        } else {
            lower = middle;
        }
    }

    lower
}

fn spell(notes: &str) -> Vec<usize> {
    let mut wall: Vec<_> = notes.iter_unsigned::<usize>().collect();
    let mut spell = Vec::new();

    for n in 0..wall.len() {
        if wall[n] > 0 {
            spell.push(n + 1);
            (n..wall.len()).step_by(n + 1).for_each(|i| wall[i] -= 1);
        }
    }

    spell
}
