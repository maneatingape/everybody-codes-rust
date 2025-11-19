use everybody_codes::event2025::quest13::*;

const EXAMPLE1: &str = "\
72
58
47
61
67";

const EXAMPLE2: &str = "\
10-15
12-13
20-21
19-23
30-37";

const EXAMPLE3: &str = "\
10000000000-15000000000
12000000000-13000000000
20000000000-21000000000
19000000000-23000000000
30000000000-37000000000";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 67);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 30);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 14520251958);
}
