use everybody_codes::story01::quest03::*;

const EXAMPLE1: &str = "
x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4";

const EXAMPLE2: &str = "\
x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3";

const EXAMPLE3: &str = "\
x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 1310);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 14);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 13659);
}
