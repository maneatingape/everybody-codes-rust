use crate::util::parse::*;

struct Plant {
    thickness: i64,
    branches: Vec<(usize, i64)>,
}

struct Input {
    free: usize,
    plants: Vec<Plant>,
}

pub fn part1(notes: &str) -> i64 {
    let Input { free, plants } = parse(notes);
    calculate(&plants, &mut vec![1; free])
}

pub fn part2(notes: &str) -> i64 {
    let (prefix, suffix) = notes.split_once("\n\n\n").unwrap();
    let Input { free, plants } = parse(prefix);
    let test_cases: Vec<_> = suffix.iter_signed().collect();
    test_cases.chunks(free).map(|chunk| calculate(&plants, &mut chunk.to_vec())).sum()
}

pub fn part3(notes: &str) -> i64 {
    let (prefix, suffix) = notes.split_once("\n\n\n").unwrap();
    let Input { free, plants } = parse(prefix);
    let test_cases: Vec<_> = suffix.iter_signed().collect();
    let mut leaf = vec![1; free];

    for Plant { branches, .. } in &plants {
        for &(index, weight) in branches {
            if weight < 0 {
                leaf[index] = 0;
            }
        }
    }

    let max = calculate(&plants, &mut leaf);
    test_cases
        .chunks(free)
        .filter_map(|chunk| {
            let energy = calculate(&plants, &mut chunk.to_vec());
            (energy > 0).then_some(max - energy)
        })
        .sum()
}

fn parse(notes: &str) -> Input {
    let mut free = 0;
    let mut plants = Vec::new();

    for block in notes.split("\n\n") {
        if block.contains("free") {
            free += 1;
        } else {
            let tokens: Vec<_> = block.iter_signed().collect();
            let thickness = tokens[1];
            let branches: Vec<_> = tokens[2..]
                .chunks(3)
                .map(|chunk| {
                    let branch = (chunk[1] - 1) as usize;
                    let weight = chunk[2];
                    (branch, weight)
                })
                .collect();

            plants.push(Plant { thickness, branches });
        }
    }

    Input { free, plants }
}

fn calculate(plants: &[Plant], energy: &mut Vec<i64>) -> i64 {
    for Plant { thickness, branches } in plants {
        let incoming = branches.iter().map(|&(branch, weight)| energy[branch] * weight).sum();
        energy.push(if incoming < *thickness { 0 } else { incoming });
    }

    *energy.last().unwrap()
}
