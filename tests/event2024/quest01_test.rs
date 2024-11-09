use everybody_codes::event2024::quest01::*;

const EXAMPLE1: &str = "ABBAC";
const EXAMPLE2: &str = "AxBCDDCAxD";
const EXAMPLE3: &str = "xBxAAABCDxCC";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 28);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 30);
}
