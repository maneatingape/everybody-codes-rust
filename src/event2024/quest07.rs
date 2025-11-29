//! Replace "S" with "=" and move to the end to simplify pattern matching.
//! The tracks are constructed so that power never drops below zero.
//!
//! For part 3 this means that the power increases by the same amount every 11 laps
//! as LCM(11, 340) = 3740.
//!
//! 2024 divided evenly by 11 equals 184, so we only need to race 11 laps
//! instead of the entire 2024 laps to find the winning plans.
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
    let plans = parse(notes);
    let mut result = BTreeMap::new();

    for (name, plan) in plans {
        let essence = score(TRACK1, 10, &plan);
        result.insert(essence, name);
    }

    result.into_values().rev().collect()
}

pub fn part2(notes: &str) -> String {
    let plans = parse(notes);
    let mut result = BTreeMap::new();

    for (name, plan) in plans {
        let essence = score(TRACK2, 10, &plan);
        result.insert(essence, name);
    }

    result.into_values().rev().collect()
}

pub fn part3(notes: &str) -> usize {
    let rival_plan = parse(notes).into_values().next().unwrap();
    let rival_score = score(TRACK3, 11, &rival_plan);
    permutations().iter().filter(|&&essence| essence > rival_score).count()
}

fn parse(notes: &str) -> HashMap<String, String> {
    notes
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(':').unwrap();
            (prefix.to_string(), suffix.replace(',', ""))
        })
        .collect()
}

fn score(track: &str, laps: usize, plan: &str) -> u64 {
    let first = track.bytes().cycle();
    let second = plan.bytes().cycle();

    let size = track.len() * laps;
    let combined = first.zip(second).take(size);

    let mut power = 10_u64;
    let mut essence = 0_u64;

    for (terrain, segment) in combined {
        let next = if terrain == b'=' { segment } else { terrain };
        match next {
            b'+' => power += 1,
            b'-' => power -= 1,
            _ => (),
        }
        essence += power;
    }

    essence
}

fn permutations() -> Vec<u64> {
    fn helper(scores: &mut Vec<u64>, plan: &mut String, plus: u8, minus: u8, equal: u8) {
        if plus + minus + equal == 0 {
            let essence = score(TRACK3, 11, plan);
            scores.push(essence);
            return;
        }
        if plus > 0 {
            plan.push('+');
            helper(scores, plan, plus - 1, minus, equal);
            plan.pop();
        }
        if minus > 0 {
            plan.push('-');
            helper(scores, plan, plus, minus - 1, equal);
            plan.pop();
        }
        if equal > 0 {
            plan.push('=');
            helper(scores, plan, plus, minus, equal - 1);
            plan.pop();
        }
    }

    let mut scores = Vec::new();
    helper(&mut scores, &mut String::new(), 5, 3, 3);
    scores
}
