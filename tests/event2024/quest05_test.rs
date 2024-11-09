use everybody_codes::event2024::quest05::*;

const EXAMPLE1: &str = "\
2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";

const EXAMPLE2: &str = "\
2 3 4 5
6 7 8 9";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 2323);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 50877075);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 6584);
}
