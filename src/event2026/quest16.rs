pub fn part1(notes: &str) -> usize {
    let (flips, _) = solve(notes, 1);
    flips
}

pub fn part2(notes: &str) -> usize {
    let (_, total) = solve(notes, 10);
    total
}

pub fn part3(notes: &str) -> usize {
    let (_, total) = solve(notes, 100);
    total
}

fn solve(notes: &str, copies: usize) -> (usize, usize) {
    let mut current = notes.as_bytes().repeat(copies);
    let mut next = Vec::with_capacity(current.len());

    let mut flips = 0;
    let mut total = 0;

    while let Some(&kind) = current.first() {
        let mut groups: Vec<_> = current
            .chunk_by(|&first, &second| (first == kind) == (second == kind))
            .skip(1)
            .collect();

        if !groups.len().is_multiple_of(2) {
            let last = groups.pop().unwrap();
            next.extend(last.iter().rev());
        }

        let mut size: usize = groups.iter().map(|group| group.len()).sum();
        let (front, back) = groups.split_at(groups.len() / 2);
        flips += front.len();

        for (i, (&front, &back)) in front.iter().zip(back.iter().rev()).enumerate() {
            if i.is_multiple_of(2) {
                next.extend(front);
            } else {
                next.extend(back.iter().rev());
            }

            total += size;
            size -= front.len() + back.len();
        }

        (current, next) = (next, current);
        current.reverse();
        next.clear();
    }

    (flips, total)
}
