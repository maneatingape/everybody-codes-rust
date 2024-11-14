use everybody_codes::event2024::quest09::*;

const EXAMPLE1: &str = "\
2
4
7
16";

const EXAMPLE2: &str = "\
33
41
55
99";

const EXAMPLE3: &str = "\
156488
352486
546212";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 10);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 10);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 10449);
}
