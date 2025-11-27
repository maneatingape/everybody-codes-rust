use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::{BTreeMap, HashMap};

pub fn part1(notes: &str) -> u32 {
    solve(notes)
}

pub fn part2(notes: &str) -> u32 {
    solve(notes)
}

pub fn part3(notes: &str) -> u32 {
    solve(notes)
}

fn solve(notes: &str) -> u32 {
    let mut map = BTreeMap::new();

    for [x1, y1, _] in notes.iter_unsigned::<u32>().chunk::<3>() {
        map.entry(x1).or_insert_with(Vec::new).push((x1, y1));
    }

    let walls: Vec<_> = map.into_values().collect();
    recurse(&walls, &mut HashMap::new(), 0, 0, 0, 0)
}

fn recurse(
    walls: &[Vec<(u32, u32)>],
    cache: &mut HashMap<(u32, u32), u32>,
    x: u32,
    y: u32,
    flaps: u32,
    index: usize,
) -> u32 {
    if index == walls.len() {
        return flaps;
    }
    if let Some(&result) = cache.get(&(x, y)) {
        return result;
    }

    let result = walls[index]
        .iter()
        .map(|&(x1, y1)| {
            let delta_x = x1 - x;
            let delta_y = y1.abs_diff(y);
            let extra = y1.saturating_sub(y) + delta_x.saturating_sub(delta_y).div_ceil(2);

            let next_x = x + delta_x;
            let next_y = y + 2 * extra - delta_x;
            recurse(walls, cache, next_x, next_y, flaps + extra, index + 1)
        })
        .min()
        .unwrap();

    cache.insert((x, y), result);
    result
}
