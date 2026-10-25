pub fn part1(notes: &str) -> u32 {
    let mut mentors = 0;
    let mut pairs = 0;

    for b in notes.trim().bytes() {
        match b {
            b'A' => mentors += 1,
            b'a' => pairs += mentors,
            _ => (),
        }
    }

    pairs
}

pub fn part2(notes: &str) -> u32 {
    let mut mentors = [0; 3];
    let mut pairs = 0;

    for b in notes.trim().bytes() {
        if b.is_ascii_uppercase() {
            mentors[usize::from(b - b'A')] += 1;
        } else {
            pairs += mentors[usize::from(b - b'a')];
        }
    }

    pairs
}

pub fn part3(notes: &str) -> i64 {
    let notes = notes.trim().as_bytes();
    let size = notes.len() as i64;
    let mut result = 0;

    for (i, &b) in (0..size).zip(notes) {
        if b.is_ascii_lowercase() {
            let mentor = b.to_ascii_uppercase();

            for j in -1000..=1000 {
                let offset = i + j;
                let wrapped = offset.rem_euclid(size) as usize;

                if notes[wrapped] == mentor {
                    let adjust = if offset < 0 { size - 1 - offset } else { offset };
                    result += 1000 - adjust / size;
                }
            }
        }
    }

    result
}
