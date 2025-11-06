use everybody_codes::event2025::quest04::*;

const EXAMPLE1: &str = "\
102
75
50
35
13";

const EXAMPLE2: &str = "\
5
7|21
18|36
27|27
10|50
10|50
11";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 15888);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 1274509803922);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 6818);
}
