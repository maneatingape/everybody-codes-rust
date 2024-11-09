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
            let mut potions = 0;
            let mut enemies = 0;

            for &b in chunk {
                if b.is_ascii_uppercase() {
                    potions += (2 * (b as i32) - 131).max(0);
                    enemies += 1;
                }
            }

            potions + enemies * (enemies - 1)
        })
        .sum()
}
