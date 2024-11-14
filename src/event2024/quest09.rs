use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    let stamps = [1, 3, 5, 10];
    dp(notes, &stamps, |beetles, brightness| beetles[brightness])
}

pub fn part2(notes: &str) -> usize {
    let stamps = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    dp(notes, &stamps, |beetles, brightness| beetles[brightness])
}

pub fn part3(notes: &str) -> usize {
    let stamps = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101];
    dp(notes, &stamps, |beetles, brightness| {
        let middle = brightness / 2;
        (middle..=middle + 50).map(|i| beetles[i] + beetles[brightness - i]).min().unwrap()
    })
}

fn dp(notes: &str, stamps: &[usize], count: fn(&[usize], usize) -> usize) -> usize {
    let list: Vec<_> = notes.iter_unsigned().collect();
    let max = 1 + list.iter().max().unwrap();

    let mut beetles = vec![max; max];
    beetles[0] = 0;

    for &stamp in stamps {
        for i in stamp..max {
            beetles[i] = beetles[i].min(beetles[i - stamp] + 1);
        }
    }

    list.iter().map(|&brightness| count(&beetles, brightness)).sum()
}
