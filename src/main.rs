use std::env::args;
use std::fs::read_to_string;

use everybody_codes::solutions;
use everybody_codes::util::ansi::*;
use everybody_codes::util::parse::*;

struct Solution {
    event: u32,
    quest: u32,
    parts: [fn(&str) -> String; 3],
}

macro_rules! run {
    ($($event:ident $($quest:ident)*),*) => {
        [$($({
            use everybody_codes::$event::$quest::*;
            Solution {
                event: stringify!($event).unsigned(),
                quest: stringify!($quest).unsigned(),
                parts: [
                    |notes| part1(notes).to_string(),
                    |notes| part2(notes).to_string(),
                    |notes| part3(notes).to_string(),
                ],
            }
        },)*)*]
    };
}

fn main() {
    let mut iter = args().skip(1).flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());
    let (event, quest) = (iter.next(), iter.next());

    solutions!(run)
        .into_iter()
        .filter(|s| event.is_none_or(|e| e == s.event))
        .filter(|s| quest.is_none_or(|q| q == s.quest))
        .for_each(|Solution { event, quest, parts }| {
            println!("{YELLOW}Event {event} Quest {quest}{RESET}");
            (1..).zip(parts).for_each(|(part, wrapper)| solve(event, quest, part, wrapper));
        });
}

fn solve(event: u32, quest: u32, part: u32, wrapper: fn(&str) -> String) {
    let path = format!("input/{event:02}/everybody_codes_e{event}_q{quest:02}_p{part}.txt");

    match read_to_string(&path) {
        Ok(notes) => println!("    Part {part}: {BOLD}{WHITE}{}{RESET}", wrapper(&notes)),
        Err(_) => eprintln!("    Part {part}: {BOLD}{WHITE}{path}{RESET} missing"),
    }
}
