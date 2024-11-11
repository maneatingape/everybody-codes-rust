use everybody_codes::event2024::quest06::*;

const EXAMPLE: &str = "\
RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@
ANT: BUG
BUG: ANT";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), "RRB@");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE), "RB@");
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE), "RB@");
}
