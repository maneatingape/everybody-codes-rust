use everybody_codes::event2026::quest16::*;

const EXAMPLE1: &str = "RRGGBBGRRGGBBG";
const EXAMPLE2: &str = "RRYGYBYG";
const EXAMPLE3: &str = "RRYGGYBBYG";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 1053);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 135154);
}
