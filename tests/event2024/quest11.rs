use everybody_codes::event2024::quest11::*;

const EXAMPLE1: &str = "\
A:B,C
B:C,A
C:A";

const EXAMPLE2: &str = "\
A:B,C
B:C,A,A
C:A";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 8);
}

#[test]
fn part2_test() {
    // No test
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 268815);
}
