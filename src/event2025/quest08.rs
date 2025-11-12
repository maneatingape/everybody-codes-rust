use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    parse(notes).iter().filter(|(s, e)| e - s == 16).count()
}

pub fn part2(notes: &str) -> u32 {
    let threads = parse(notes);
    threads
        .iter()
        .enumerate()
        .flat_map(|(i, &(s1, e1))| {
            threads[0..i].iter().map(move |&(s2, e2)| overlap(s1, e1, s2, e2))
        })
        .sum()
}

pub fn part3(notes: &str) -> u32 {
    let threads = parse(notes);
    let mut result = 0;

    for e1 in 2..257 {
        for s1 in 1..e1 {
            let cuts = threads.iter().map(|&(s2, e2)| overlap(s1, e1, s2, e2)).sum();
            result = result.max(cuts);
        }
    }

    result
}

fn parse(notes: &str) -> Vec<(u32, u32)> {
    let nails: Vec<_> = notes.iter_unsigned::<u32>().collect();
    nails.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect()
}

fn overlap(s1: u32, e1: u32, s2: u32, e2: u32) -> u32 {
    u32::from(
        (s1 == s2 && e1 == e2)
            || (s2 > s1 && s2 < e1 && e2 > e1)
            || (e2 > s1 && e2 < e1 && s2 < s1),
    )
}
