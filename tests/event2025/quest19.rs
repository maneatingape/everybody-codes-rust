use everybody_codes::event2025::quest19::*;

const EXAMPLE1: &str = "\
7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2";

const EXAMPLE2: &str = "\
7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 24);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 22);
}

#[test]
fn part3_test() {
    // No example data.
}
