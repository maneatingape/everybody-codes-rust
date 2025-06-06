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
            let mut iter = arg.split("::").map(String::from);
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions = empty()
        .chain(event2024())
        .chain(story01())
        .filter(|solution| event.as_ref().is_none_or(|e| *e == solution.event))
        .filter(|solution| quest.as_ref().is_none_or(|q| *q == solution.quest));

    // Pretty print output for each solution
    for Solution { event, quest, part1, part2, part3 } in solutions {
        println!("{YELLOW}{event} {quest}{RESET}");
        solve(&event, &quest, 1, part1);
        solve(&event, &quest, 2, part2);
        solve(&event, &quest, 3, part3);
    }
}

fn solve(event: &str, quest: &str, part: u32, wrapper: fn(&str) -> String) {
    let first: u32 = event.unsigned();
    let second: u32 = quest.unsigned();
    let path = format!("input/{event}/everybody_codes_e{first}_q{second:02}_p{part}.txt");

    if let Ok(notes) = read_to_string(&path) {
        println!("    Part {part}: {BOLD}{WHITE}{}{RESET}", wrapper(&notes));
    } else {
        eprintln!("    Part {part}: {BOLD}{WHITE}{path}{RESET} missing");
    }
}

struct Solution {
    event: String,
    quest: String,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
    part3: fn(&str) -> String,
}

macro_rules! run {
    ($event:tt $($quest:tt),*) => {
        fn $event() -> Vec<Solution> {
            vec![$({
                use $event::$quest::*;

                let event = stringify!($event).to_string();
                let quest = stringify!($quest).to_string();
                let part1 = |notes: &str| part1(notes).to_string();
                let part2 = |notes: &str| part2(notes).to_string();
                let part3 = |notes: &str| part3(notes).to_string();

                Solution { event, quest, part1, part2, part3 }
            },)*]
        }
    }
}

run!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15, quest16, quest17, quest18, quest19, quest20
);

run!(story01
    quest01, quest02, quest03
);
