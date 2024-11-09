use everybody_codes::event2024::quest04::*;

const EXAMPLE1: &str = "\
3
4
7
8";

const EXAMPLE2: &str = "\
2
4
5
6
8";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 10);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 15);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 8);
}
