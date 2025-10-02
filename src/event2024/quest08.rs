use crate::util::parse::*;

pub fn part1(notes: &str) -> u64 {
    let blocks: u64 = notes.unsigned();
    let height = blocks.isqrt() + 1;
    let width = 2 * height - 1;
    let missing = height * height - blocks;
    width * missing
}

pub fn part2(notes: &str) -> u64 {
    let supply = 20240000;
    let priests: u64 = notes.unsigned();

    let mut width = 1;
    let mut layer = 1;
    let mut blocks = 1;

    while blocks < supply {
        width += 2;
        layer = (layer * priests) % 1111;
        blocks += width * layer;
    }

    width * (blocks - supply)
}

pub fn part3(notes: &str) -> u64 {
    let supply = 202400000;
    let priests: u64 = notes.unsigned();

    let mut width = 1;
    let mut layer = 1;
    let mut blocks = 1;
    let mut height = 1;
    let mut layers = Vec::new();

    while blocks < supply {
        layers.push(layer);
        width += 2;
        layer = (layer * priests) % 10 + 10;
        blocks += width * layer;
        height += layer;
    }

    blocks += (priests * width * height) % 10;

    for layer in layers {
        blocks -= 2 * ((priests * width * height) % 10);
        height -= layer;
    }

    blocks - supply
}
