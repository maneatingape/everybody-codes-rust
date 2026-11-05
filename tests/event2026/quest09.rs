use everybody_codes::event2026::quest09::*;

const EXAMPLE1: &str = "\
id=1, growth=3, limit=10
id=2, growth=5, limit=20
id=3, growth=7, limit=30";

const EXAMPLE2: &str = "\
id=1, growth=3000000003, limit=10000000000
id=2, growth=5000000005, limit=20000000000
id=3, growth=7000000007, limit=30000000000";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 27);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 42);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 216000000238);
}
