use everybody_codes::event2025::quest15::*;

const EXAMPLE1: &str = "R3,R4,L3,L4,R3,R6,R9";
const EXAMPLE2: &str = "L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 6);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 16);
}

#[test]
fn part3_test() {
    // No example data.
}
