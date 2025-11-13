use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    parse(notes).iter().filter(|(s, e)| e - s == 16).count()
}

pub fn part2(notes: &str) -> i32 {
    let mut links = vec![vec![]; 257];
    let mut freq = vec![0; 258];
    let mut result = 0;

    for (start, end) in parse(notes) {
        links[start].push(end);
    }

    for (start, ends) in links.iter().enumerate() {
        for &end in ends {
            result += freq[start + 1..end].iter().sum::<i32>();
        }

        for &end in ends {
            freq[end] += 1;
        }
    }

    result
}

pub fn part3(notes: &str) -> i32 {
    let mut links = vec![vec![]; 257];
    let mut delta = vec![0; 258];
    let mut result = 0;

    for (start, end) in parse(notes) {
        links[start].push(end);
        delta[start + 1] += 1;
        delta[end] -= 1;
    }

    for start in 1..255 {
        for &end in &links[start] {
            delta[end] += 2;
            delta[end + 1] -= 1;
        }

        for &end in &links[start - 1] {
            delta[end] -= 1;
            delta[end + 1] += 2;
        }

        let mut cuts = 0;

        for &next in &delta[start + 2..257] {
            cuts += next;
            result = result.max(cuts);
        }
    }

    result
}

fn parse(notes: &str) -> Vec<(usize, usize)> {
    let nails: Vec<usize> = notes.iter_unsigned().collect();
    nails.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect()
}
