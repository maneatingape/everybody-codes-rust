use everybody_codes::event2024::quest12::*;

const EXAMPLE1: &str = "\
.............
.C...........
.B......T....
.A......T.T..
=============";

const EXAMPLE2: &str = "\
.............
.C...........
.B......H....
.A......T.H..
=============";

const EXAMPLE3: &str = "\
6 5
6 7
10 5";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 13);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 22);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 11);
}
