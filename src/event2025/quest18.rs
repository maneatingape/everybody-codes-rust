use crate::util::iter::*;
use crate::util::parse::*;

struct Plant {
    thickness: i64,
    branches: Vec<(usize, i64)>,
}

struct Input {
    free: usize,
    plants: Vec<Plant>,
    test_cases: Vec<i64>,
}

pub fn part1(notes: &str) -> i64 {
    let Input { free, plants, .. } = parse(notes);
    calculate(&plants, vec![1; free])
}

pub fn part2(notes: &str) -> i64 {
    let Input { free, plants, test_cases: tests } = parse(notes);
    tests.chunks(free).map(|chunk| calculate(&plants, chunk.to_vec())).sum()
}

pub fn part3(notes: &str) -> i64 {
    let Input { free, plants, test_cases } = parse(notes);
    let mut leaf = vec![1; free];

    for &(index, weight) in plants.iter().flat_map(|plant| &plant.branches) {
        if weight < 0 {
            leaf[index] = 0;
        }
    }

    let max = calculate(&plants, leaf);
    test_cases
        .chunks(free)
        .map(|chunk| calculate(&plants, chunk.to_vec()))
        .filter(|&energy| energy > 0)
        .map(|energy| max - energy)
        .sum()
}

fn parse(notes: &str) -> Input {
    let (prefix, suffix) = notes.split_once("\n\n\n").unwrap_or((notes, ""));
    let test_cases = suffix.iter_signed().collect();

    let mut free = 0;
    let mut plants = Vec::new();

    for block in prefix.split("\n\n") {
        if block.contains("free") {
            free += 1;
        } else {
            let mut tokens = block.iter_signed();
            let thickness = tokens.nth(1).unwrap();
            let branches = tokens
                .chunk::<3>()
                .map(|[_, branch, weight]| ((branch - 1) as usize, weight))
                .collect();

            plants.push(Plant { thickness, branches });
        }
    }

    Input { free, plants, test_cases }
}

fn calculate(plants: &[Plant], mut energy: Vec<i64>) -> i64 {
    for Plant { thickness, branches } in plants {
        let incoming = branches.iter().map(|&(branch, weight)| energy[branch] * weight).sum();
        energy.push(if incoming < *thickness { 0 } else { incoming });
    }

    *energy.last().unwrap()
}
