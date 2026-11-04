use crate::util::grid::*;
use crate::util::point::*;

const DIRECTIONS: [Point; 6] = [
    Point::new(1, 1),
    Point::new(-1, 1),
    Point::new(-2, 0),
    Point::new(-1, -1),
    Point::new(1, -1),
    Point::new(2, 0),
];

pub fn part1(notes: &str) -> u32 {
    solve(notes, 1)
}

pub fn part2(notes: &str) -> u32 {
    solve(notes, 6)
}

pub fn part3(notes: &str) -> u32 {
    let (mut grid, edges) = parse(notes);
    let mut energy = 0;
    let mut warriors = 0;
    let mut rounds = edges.iter().cycle();
    let mut seeds = [const { Vec::new() }; 4];

    while warriors < 100_000 {
        let (direction, edge) = rounds.next().unwrap();

        for &start in edge {
            let mut shadow = 0;
            let mut point = start;

            while grid.contains(point) {
                let height = grid[point];

                if height.is_ascii_digit() && height > shadow {
                    shadow = height;
                    energy += 1;

                    if height == b'9' {
                        warriors += 1;
                        grid[point] = b'X';
                        seeds[0].push(point);
                    } else {
                        grid[point] += 1;
                    }
                }

                point += *direction;
                shadow = shadow.saturating_sub(1);
            }
        }

        seeds.rotate_right(1);
        seeds[0].drain(..).for_each(|seed| grid[seed] = b'1');
    }

    energy
}

fn solve(notes: &str, rounds: usize) -> u32 {
    let (grid, edges) = parse(notes);
    edges.iter().take(rounds).map(|(direction, edge)| round(&grid, *direction, edge)).sum()
}

fn parse(notes: &str) -> (Grid<u8>, [(Point, Vec<Point>); 6]) {
    let grid = Grid::parse(notes);
    let hexagon = |point| grid.contains(point) && grid[point].is_ascii_digit();
    let edges = DIRECTIONS.map(|direction| {
        (
            direction,
            grid.points().filter(|&point| hexagon(point) && !hexagon(point - direction)).collect(),
        )
    });

    (grid, edges)
}

fn round(grid: &Grid<u8>, direction: Point, edge: &[Point]) -> u32 {
    let mut energy = 0;

    for &start in edge {
        let mut shadow = 0;
        let mut point = start;

        while grid.contains(point) {
            energy += u32::from(grid[point] > shadow);
            shadow = shadow.max(grid[point]).saturating_sub(1);
            point += direction;
        }
    }

    energy
}
