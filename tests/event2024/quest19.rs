use everybody_codes::event2024::quest19::*;

const EXAMPLE1: &str = "\
LR

>-IN-
-----
W---<";

const EXAMPLE2: &str = "\
RRLL

A.VI..>...T
.CC...<...O
.....EIB.R.
.DHB...YF..
.....F..G..
D.H........";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), "WIN");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), "VICTORY");
}

#[test]
fn part3_test() {
    // No test
}
