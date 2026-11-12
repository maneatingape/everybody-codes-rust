use std::collections::{HashMap, HashSet};

use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;

const OUTSIDE: u8 = 0;

pub fn part1(notes: &str) -> u32 {
    let graph = weights(notes);
    graph.values().flatten().map(|&(_, weight)| weight).sum::<u32>() / 2
}

pub fn part2(notes: &str) -> u32 {
    let graph = weights(notes);
    prim(&graph, &[])
}

pub fn part3(notes: &str) -> u32 {
    let graph = weights(notes);
    let kinds: Vec<_> = graph.keys().copied().filter(|&kind| kind != OUTSIDE).collect();
    let mut bindings = u32::MAX;

    for (i, &first) in kinds.iter().enumerate() {
        for &second in &kinds[i + 1..] {
            bindings = bindings.min(prim(&graph, &[first, second]));
        }
    }

    bindings
}

fn weights(notes: &str) -> HashMap<u8, Vec<(u8, u32)>> {
    let grid = Grid::parse(notes);
    let mut weights = HashMap::new();
    let mut graph = HashMap::new();

    for point in grid.points() {
        let kind = grid[point];

        for next in ORTHOGONAL.map(|o| point + o) {
            let other = if grid.contains(next) { grid[next] } else { OUTSIDE };
            if kind > other {
                *weights.entry((kind, other)).or_insert(0) += 1;
            }
        }
    }

    for ((from, to), weight) in weights {
        graph.entry(from).or_insert_with(Vec::new).push((to, weight));
        graph.entry(to).or_insert_with(Vec::new).push((from, weight));
    }

    graph
}

fn prim(graph: &HashMap<u8, Vec<(u8, u32)>>, free: &[u8]) -> u32 {
    let mut todo = MinHeap::from([(0, OUTSIDE)]);
    let mut seen = HashSet::new();
    let mut total = 0;

    while let Some((weight, kind)) = todo.pop() {
        if !seen.insert(kind) {
            continue;
        }

        total += weight;

        for &(next, weight) in &graph[&kind] {
            if !seen.contains(&next) {
                let cost = if free.contains(&kind) || free.contains(&next) { 0 } else { weight };
                todo.push(cost, next);
            }
        }
    }

    total
}
