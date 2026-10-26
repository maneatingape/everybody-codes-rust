use everybody_codes::event2026::quest01::*;

const EXAMPLE1: &str = "3,11,12,16,24,25";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 11);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 9);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE1), 61);
}
