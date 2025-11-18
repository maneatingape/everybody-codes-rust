use everybody_codes::util::ansi::*;
use everybody_codes::util::parse::*;
use std::env::args;
use std::fs::read_to_string;

fn main() {
    // Parse command line options
    let mut iter = args().flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());
    let (event, quest) = (iter.next(), iter.next());

    let solutions = [event2024(), event2025(), story01(), story02()];

    // Filter solutions then pretty print output.
    solutions
        .into_iter()
        .flatten()
        .filter(|s| event.is_none_or(|e| e == s.event))
        .filter(|s| quest.is_none_or(|q| q == s.quest))
        .for_each(|Solution { event, quest, part1, part2, part3 }| {
            println!("{YELLOW}Event {event} Quest {quest}{RESET}");
            solve(event, quest, 1, part1);
            solve(event, quest, 2, part2);
            solve(event, quest, 3, part3);
        });
}

fn solve(event: u32, quest: u32, part: u32, wrapper: fn(&str) -> String) {
    let path = format!("input/{event:02}/everybody_codes_e{event}_q{quest:02}_p{part}.txt");

    if let Ok(notes) = read_to_string(&path) {
        println!("    Part {part}: {BOLD}{WHITE}{}{RESET}", wrapper(&notes));
    } else {
        eprintln!("    Part {part}: {BOLD}{WHITE}{path}{RESET} missing");
    }
}

struct Solution {
    event: u32,
    quest: u32,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
    part3: fn(&str) -> String,
}

macro_rules! run {
    ($event:tt $($quest:tt),*) => {
        fn $event() -> Vec<Solution> {
            vec![$({
                use everybody_codes::$event::$quest::*;
                Solution {
                    event: stringify!($event).unsigned(),
                    quest: stringify!($quest).unsigned(),
                    part1: |notes: &str| part1(notes).to_string(),
                    part2: |notes: &str| part2(notes).to_string(),
                    part3: |notes: &str| part3(notes).to_string(),
                }
            },)*]
        }
    }
}

run!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15, quest16, quest17, quest18, quest19, quest20
);

run!(event2025
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12
);

run!(story01
    quest01, quest02, quest03
);

run!(story02
    quest01, quest02, quest03
);
