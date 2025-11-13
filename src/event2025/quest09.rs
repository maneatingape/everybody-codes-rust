use std::collections::{HashMap, HashSet, VecDeque};

struct Duck {
    size: u32,
    lo: u128,
    hi: u128,
}

impl Duck {
    fn from(line: &str) -> Self {
        let (_, suffix) = line.split_once(':').unwrap();

        let size = suffix.len() as u32;
        let lo = suffix.bytes().fold(0, |lo, b| (lo << 1) | u128::from(b == b'G' || b == b'T'));
        let hi = suffix.bytes().fold(0, |hi, b| (hi << 1) | u128::from(b == b'C' || b == b'T'));

        Duck { size, lo, hi }
    }

    fn similar(&self, other: &Duck) -> u32 {
        self.size - ((self.lo ^ other.lo) | (self.hi ^ other.hi)).count_ones()
    }

    fn parents(&self, first: &Duck, second: &Duck) -> bool {
        let lo = (self.lo ^ first.lo) & (self.lo ^ second.lo);
        let hi = (self.hi ^ first.hi) & (self.hi ^ second.hi);
        (lo | hi) == 0
    }
}

pub fn part1(notes: &str) -> u32 {
    let mut ducks: Vec<_> = notes.lines().map(Duck::from).collect();

    while !ducks[0].parents(&ducks[1], &ducks[2]) {
        ducks.rotate_left(1);
    }

    ducks[0].similar(&ducks[1]) * ducks[0].similar(&ducks[2])
}

pub fn part2(notes: &str) -> u32 {
    let ducks: Vec<_> = notes.lines().map(Duck::from).collect();
    let mut result = 0;

    'outer: for child in 0..ducks.len() {
        for first in 0..ducks.len() {
            for second in first + 1..ducks.len() {
                if first != child
                    && second != child
                    && ducks[child].parents(&ducks[first], &ducks[second])
                {
                    result +=
                        ducks[child].similar(&ducks[first]) * ducks[child].similar(&ducks[second]);
                    continue 'outer;
                }
            }
        }
    }

    result
}

pub fn part3(notes: &str) -> usize {
    let ducks: Vec<_> = notes.lines().map(Duck::from).collect();

    let mut graph = HashMap::new();
    let mut link = |from, to| {
        graph.entry(from).or_insert_with(Vec::new).push(to);
        graph.entry(to).or_insert_with(Vec::new).push(from);
    };

    'outer: for child in 0..ducks.len() {
        for first in 0..ducks.len() {
            for second in first + 1..ducks.len() {
                if first != child
                    && second != child
                    && ducks[child].parents(&ducks[first], &ducks[second])
                {
                    link(first, child);
                    link(second, child);
                    continue 'outer;
                }
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut remaining: HashSet<_> = (0..ducks.len()).collect();
    let mut groups = Vec::new();

    while let Some(&start) = remaining.iter().next() {
        todo.push_back(start);
        remaining.remove(&start);

        let mut size = 0;
        let mut scales = 0;

        while let Some(duck) = todo.pop_front() {
            size += 1;
            scales += duck;

            if let Some(relatives) = graph.get(&duck) {
                for &next in relatives {
                    if remaining.remove(&next) {
                        todo.push_back(next);
                    }
                }
            }
        }

        groups.push((size, scales));
    }

    let &(size, scales) = groups.iter().max_by_key(|(size, _)| size).unwrap();
    size + scales
}
