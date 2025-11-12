use everybody_codes::event2025::quest08::*;

const EXAMPLE1: &str = "1,17,5,21,29,13,1,25,9";
const EXAMPLE2: &str = "1,5,2,6,8,4,1,7,3,5,7,8,2";
const EXAMPLE3: &str = "1,5,2,6,8,4,1,7,3,6";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 21);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 7);
}
