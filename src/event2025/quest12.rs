use crate::util::grid::*;
use crate::util::point::*;
use std::collections::{HashSet, VecDeque};

pub fn part1(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    ignite(&grid, vec![ORIGIN])
}

pub fn part2(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    ignite(&grid, vec![ORIGIN, Point::new(grid.width - 1, grid.height - 1)])
}

pub fn part3(notes: &str) -> u32 {
    let grid = Grid::parse(notes);
    let points =
        (0..grid.height).flat_map(|y| (0..grid.width).map(move |x| Point::new(x, y))).collect();
    ignite(&grid, points)
}

fn ignite(grid: &Grid<u8>, mut points: Vec<Point>) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(usize::MAX);
    let mut sets = Vec::new();
    let mut sizes = Vec::new();

    points.sort_unstable_by_key(|&point| grid[point]);

    for start in points {
        if seen[start] == usize::MAX {
            let id = sets.len();
            let mut set = HashSet::from([id]);
            let mut size = 1;

            todo.push_back(start);
            seen[start] = id;

            while let Some(point) = todo.pop_front() {
                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] <= grid[point] {
                        if seen[next] == usize::MAX {
                            todo.push_back(next);
                            seen[next] = id;
                            size += 1;
                        } else if set.insert(seen[next]) {
                            set.extend(&sets[seen[next]]);
                        }
                    }
                }
            }

            sets.push(set);
            sizes.push(size);
        }
    }

    (0..3)
        .scan(HashSet::new(), |used, _| {
            let (set, total) = sets
                .iter()
                .map(|set| (set, set.difference(used).map(|&i| sizes[i]).sum::<u32>()))
                .max_by_key(|&(_, total)| total)
                .unwrap();
            used.extend(set);
            Some(total)
        })
        .sum()
}
