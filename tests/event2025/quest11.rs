use everybody_codes::event2025::quest11::*;

const EXAMPLE1: &str = "
9
1
1
4
9
6";

const EXAMPLE2: &str = "
805
706
179
48
158
150
232
885
598
524
423";

const EXAMPLE3: &str = "
1000000000000
2000000000000
3000000000000
4000000000000
5000000000000";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 109);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 1579);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 3_000_000_000_000);
}
