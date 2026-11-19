pub fn part1(notes: &str) -> u64 {
    let colors = pairs(notes).map(|(first, second)| mix(first, second));
    top_three(colors)
}

pub fn part2(notes: &str) -> u64 {
    let mut cubes = 0;

    permutations(*b"CYMK", 0, &mut |ink| {
        let colors = pairs(notes).map(|(first, second)| mix(first, ink[to_index(second)]));
        cubes = cubes.max(top_three(colors));
    });

    cubes
}

pub fn part3(notes: &str) -> usize {
    let sum = notes.bytes().filter(|b| b"CYM".contains(b)).count();
    (0..3).map(|i| (sum + i) / 3).product()
}

fn pairs(notes: &str) -> impl Iterator<Item = (u8, u8)> {
    let (first, second) = notes.split_once("\n\n").unwrap();
    first.bytes().zip(second.bytes()).filter(|&(first, _)| first != b'\n')
}

fn mix(first: u8, second: u8) -> u8 {
    if first == b'K' || second == b'K' { b'K' } else { first + second }
}

fn top_three(iter: impl Iterator<Item = u8>) -> u64 {
    let mut frequency = [0; 256];
    iter.for_each(|key| frequency[to_index(key)] += 1);
    frequency.sort_unstable();
    frequency.into_iter().rev().take(3).product()
}

fn permutations<F: FnMut([u8; 4])>(mut array: [u8; 4], i: usize, f: &mut F) {
    if i == 4 {
        f(array);
    } else {
        for j in i..4 {
            array.swap(i, j);
            permutations(array, i + 1, f);
            array.swap(i, j);
        }
    }
}

fn to_index(b: u8) -> usize {
    (b - b'1') as usize
}
