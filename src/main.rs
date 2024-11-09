use everybody_codes::util::ansi::*;
use everybody_codes::util::parse::*;
use everybody_codes::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;

fn main() {
    // Parse command line options
    let (event, quest) = match args().nth(1) {
        Some(arg) => {
            let str = arg.as_str();
            let mut iter = str.iter_unsigned();
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions = empty()
        .chain(event2024())
        .filter(|solution| event.is_none_or(|e: u32| e == solution.event))
        .filter(|solution| quest.is_none_or(|q: u32| q == solution.quest));

    // Pretty print output for each solution
    for Solution { event, quest, part1, part2, part3 } in solutions {
        println!("{YELLOW}{event} Quest {quest}{RESET}");
        solve(event, quest, 1, part1);
        solve(event, quest, 2, part2);
        solve(event, quest, 3, part3);
    }
}

fn solve(event: u32, quest: u32, part: u32, wrapper: fn(&str) -> String) {
    let path = format!("input/event{event}/everybody_codes_e{event}_q{quest:02}_p{part}.txt");

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

macro_rules! solution {
    ($event:tt, $quest:tt) => {{
        use $event::$quest::*;

        let event = stringify!($event).unsigned();
        let quest = stringify!($quest).unsigned();
        let part1 = |notes: &str| part1(notes).to_string();
        let part2 = |notes: &str| part2(notes).to_string();
        let part3 = |notes: &str| part3(notes).to_string();

        Solution { event, quest, part1, part2, part3 }
    }};
}

fn event2024() -> Vec<Solution> {
    vec![
        solution!(event2024, quest01),
        solution!(event2024, quest02),
        solution!(event2024, quest03),
        solution!(event2024, quest04),
        solution!(event2024, quest05),
    ]
}
