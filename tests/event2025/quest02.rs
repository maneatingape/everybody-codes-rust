use everybody_codes::event2025::quest02::*;

const EXAMPLE1: &str = "A=[25,9]";
const EXAMPLE2: &str = "A=[35300,-64910]";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), "[357,862]");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 4076);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 406954);
}
