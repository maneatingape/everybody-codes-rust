use everybody_codes::story04::quest03::*;

const EXAMPLE1: &str = "\
width=30
height=10
horizontal-offsets=10011
vertical-offsets=11011";

const EXAMPLE2: &str = "\
width=100
height=70
horizontal-offsets=111101111101101111000100100110
vertical-offsets=110100001110111011101000001111";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 27);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 15);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 269);
}
