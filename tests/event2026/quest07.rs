use everybody_codes::event2026::quest07::*;

const EXAMPLE1: &str = "\
id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 102);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 1316);
}

#[test]
fn part3_test() {
    assert_eq!(part3_testable(EXAMPLE1, 100), 187);
}
