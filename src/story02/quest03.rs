use crate::util::grid::*;
use crate::util::parse::*;
use crate::util::point::*;

struct Dice {
    faces: Vec<i32>,
    seed: usize,
    pulse: usize,
    index: usize,
    roll_number: usize,
}

impl Dice {
    fn from(line: &str) -> Dice {
        let mut faces: Vec<_> = line.iter_signed::<i32>().skip(1).collect();
        let seed = faces.pop().unwrap() as usize;
        Dice { faces, seed, pulse: seed, index: 0, roll_number: 1 }
    }

    fn roll(&mut self) -> i32 {
        let spin = self.roll_number * self.pulse;

        self.index = (self.index + spin) % self.faces.len();
        self.pulse = (self.pulse + spin) % self.seed + (1 + self.roll_number + self.seed);
        self.roll_number += 1;

        self.faces[self.index]
    }
}

pub fn part1(notes: &str) -> usize {
    let mut dice: Vec<_> = notes.lines().map(Dice::from).collect();
    let mut total = 0;
    let mut rolls = 0;

    while total < 10_000 {
        total += dice.iter_mut().map(Dice::roll).sum::<i32>();
        rolls += 1;
    }

    rolls
}

pub fn part2(notes: &str) -> String {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let dice: Vec<_> = prefix.lines().map(Dice::from).collect();
    let track: Vec<_> = suffix.trim().bytes().map(|b| b.to_decimal() as i32).collect();

    let mut result: Vec<_> = dice
        .into_iter()
        .enumerate()
        .map(|(id, mut die)| {
            let mut rolls = 0;

            for &current in &track {
                while die.roll() != current {
                    rolls += 1;
                }
            }

            (id + 1, rolls)
        })
        .collect();

    result.sort_unstable_by_key(|&p| p.1);
    result.iter().map(|&p| p.0.to_string()).collect::<Vec<_>>().join(",")
}

pub fn part3(notes: &str) -> usize {
    let (prefix, suffix) = notes.split_once("\n\n").unwrap();
    let dice: Vec<_> = prefix.lines().map(Dice::from).collect();
    let grid = Grid::parse(suffix);

    let mut todo = Vec::with_capacity(100_000);
    let mut next = Vec::with_capacity(100_000);

    let mut seen = grid.same_size_with(0);
    let mut version = 0;

    for mut die in dice {
        let roll = die.roll() as u8 + b'0';
        version += 1;

        for x in 0..grid.width {
            for y in 0..grid.height {
                let point = Point::new(x, y);

                if grid[point] == roll {
                    todo.push(point);
                    seen[point] = version;
                }
            }
        }

        while !todo.is_empty() {
            let roll = die.roll() as u8 + b'0';
            version += 1;

            for point in todo.drain(..) {
                if grid[point] == roll && seen[point] != version {
                    next.push(point);
                    seen[point] = version;
                }

                for neighbor in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(neighbor)
                        && grid[neighbor] == roll
                        && seen[neighbor] != version
                    {
                        next.push(neighbor);
                        seen[neighbor] = version;
                    }
                }
            }

            (todo, next) = (next, todo);
        }
    }

    seen.bytes.iter().filter(|&&v| v > 0).count()
}
