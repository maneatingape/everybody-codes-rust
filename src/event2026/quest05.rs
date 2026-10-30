use crate::util::grid::*;
use crate::util::point::*;

pub fn part1(notes: &str) -> usize {
    let grid = &mut Grid::parse(notes);
    land(grid);
    count(grid, b'#')
}

pub fn part2(notes: &str) -> usize {
    let grid = &mut Grid::parse(notes);
    land(grid);
    sand(grid);
    count(grid, b'=')
}

pub fn part3(notes: &str) -> usize {
    let grid = &mut Grid::parse(notes);
    land(grid);
    sand(grid);

    tree(grid, |grid, point| ORTHOGONAL.iter().all(|&o| around(grid, point + o, b"#") == 8));

    for _ in 0..2 {
        tree(grid, |grid, point| around(grid, point, b"@") >= 4 && around(grid, point, b"=") == 0);
    }

    count(grid, b'@')
}

fn land(grid: &mut Grid<u8>) {
    let start = Point::new(grid.width / 2, grid.height / 2);
    spread(grid, vec![start], b'#', b"#1");
}

fn sand(grid: &mut Grid<u8>) {
    let edge: Vec<_> = grid
        .points()
        .filter(|&point| grid[point] == b'#' && around(grid, point, b"#") < 8)
        .collect();
    spread(grid, edge, b'=', b"=");
}

fn tree(grid: &mut Grid<u8>, rule: fn(&Grid<u8>, Point) -> bool) {
    let grow: Vec<_> =
        grid.points().filter(|&point| grid[point] == b'#' && rule(grid, point)).collect();
    grow.iter().for_each(|&point| grid[point] = b'@');
}

fn spread(grid: &mut Grid<u8>, mut todo: Vec<Point>, kind: u8, wanted: &[u8]) {
    todo.iter().for_each(|&point| grid[point] = kind);

    while let Some(point) = todo.pop() {
        for next in DIAGONAL.map(|d| point + d) {
            if grid.contains(next) && grid[next] != kind && around(grid, next, wanted) >= 5 {
                grid[next] = kind;
                todo.push(next);
            }
        }
    }
}

fn around(grid: &Grid<u8>, point: Point, kinds: &[u8]) -> usize {
    DIAGONAL
        .iter()
        .map(|&d| point + d)
        .filter(|&next| grid.contains(next) && kinds.contains(&grid[next]))
        .count()
}

fn count(grid: &Grid<u8>, kind: u8) -> usize {
    grid.bytes.iter().filter(|&&b| b == kind).count()
}
