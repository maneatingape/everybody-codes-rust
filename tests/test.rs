// Templates

// pub fn part1(_notes: &str) -> u32 {
//     123
// }

// pub fn part2(_notes: &str) -> u32 {
//     456
// }

// pub fn part3(_notes: &str) -> u32 {
//     789
// }

// use everybody_codes::event2025::quest00::*;

// const EXAMPLE1: &str = "";
// const EXAMPLE2: &str = "";
// const EXAMPLE3: &str = "";

// #[test]
// fn part1_test() {
//     assert_eq!(part1(EXAMPLE1), 123);
// }

// #[test]
// fn part2_test() {
//     assert_eq!(part2(EXAMPLE2), 456);
// }

// #[test]
// fn part3_test() {
//     assert_eq!(part3(EXAMPLE3), 789);
// }

macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

test!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15, quest16, quest17, quest18, quest19, quest20
);

test!(event2025
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08
);

test!(story01
    quest01, quest02, quest03
);

test!(story02
    quest01, quest02, quest03
);
