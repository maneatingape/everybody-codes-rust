use std::collections::{BTreeMap, HashMap};

const TRACK1: &str = "=";

const TRACK2: &str = "\
-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=---=++==--\
+++==++=+=--==++==+++=++=+++=--=+=-=+=-+=-+=-+-=+=-=+=-+++=+==++++==---=+=+=-=";

const TRACK3: &str = "\
+=+++===-+++++=-==+--+=+===-++=====+--===++=-==+=++====-==-===+=+=--==++=+========-==\
=====++--+++=-++=-+=+==-=++=--+=-====++--+=-==++======+=++=-+==+=-==++=-=-=---++=-=++\
==++===--==+===++===---+++==++=+=-=====+==++===--==-==+++==+++=++=+===--==++--===+===\
==-=++====-+=-+--=+++=-+-===++====+++--=++====+=-=+===+=====-+++=+==++++==----=+=+=-=";

pub fn part1(notes: &str) -> String {
    ranking(notes, TRACK1)
}

pub fn part2(notes: &str) -> String {
    ranking(notes, TRACK2)
}

pub fn part3(notes: &str) -> usize {
    let rival_plan = parse(notes).into_values().next().unwrap();
    let rival_score = score(TRACK3, 11, &rival_plan);
    permutations().iter().filter(|&&essence| essence > rival_score).count()
}

fn parse(notes: &str) -> HashMap<&str, String> {
    notes
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(':').unwrap();
            (prefix, suffix.replace(',', ""))
        })
        .collect()
}

fn ranking(notes: &str, track: &str) -> String {
    let result: BTreeMap<_, _> =
        parse(notes).into_iter().map(|(name, plan)| (score(track, 10, &plan), name)).collect();
    result.into_values().rev().collect()
}

fn score(track: &str, laps: usize, plan: &str) -> u64 {
    track
        .bytes()
        .cycle()
        .zip(plan.bytes().cycle())
        .take(track.len() * laps)
        .scan(10, |power, (terrain, segment)| {
            match if terrain == b'=' { segment } else { terrain } {
                b'+' => *power += 1,
                b'-' => *power -= 1,
                _ => (),
            }
            Some(*power)
        })
        .sum()
}

fn permutations() -> Vec<u64> {
    fn helper(scores: &mut Vec<u64>, plan: &mut String, remaining: [u8; 3]) {
        if remaining == [0; 3] {
            scores.push(score(TRACK3, 11, plan));
            return;
        }

        for (index, segment) in "+-=".chars().enumerate() {
            if remaining[index] > 0 {
                let mut next = remaining;
                next[index] -= 1;

                plan.push(segment);
                helper(scores, plan, next);
                plan.pop();
            }
        }
    }

    let mut scores = Vec::new();
    helper(&mut scores, &mut String::new(), [5, 3, 3]);
    scores
}
