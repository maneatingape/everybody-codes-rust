pub fn part1(notes: &str) -> i32 {
    score(notes, 1)
}

pub fn part2(notes: &str) -> i32 {
    score(notes, 2)
}

pub fn part3(notes: &str) -> i32 {
    score(notes, 3)
}

fn score(notes: &str, size: usize) -> i32 {
    notes
        .as_bytes()
        .chunks(size)
        .map(|chunk| {
            let runes = chunk.iter().filter(|b| b.is_ascii_uppercase());

            let (potions, enemies) = runes.fold((0, 0), |(potions, enemies), &b| {
                let rank = i32::from(b - b'A');
                (potions + (2 * rank - 1).max(0), enemies + 1)
            });

            potions + enemies * (enemies - 1)
        })
        .sum()
}
