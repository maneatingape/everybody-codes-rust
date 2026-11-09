use everybody_codes::event2026::quest11::*;

const EXAMPLE1: &str = "\
portals: 10

5 10
2 7";

const EXAMPLE2: &str = "\
portals: 32

1 10
20 30
15 29
13 10";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 60);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 6);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 854);
}
