use everybody_codes::event2025::quest16::*;

const EXAMPLE1: &str = "1,2,3,5,9";
const EXAMPLE2: &str = "1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2";
const EXAMPLE3: &str = "1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 193);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 270);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 94439495762954);
}
