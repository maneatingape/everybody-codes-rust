use everybody_codes::event2025::quest03::*;

const EXAMPLE1: &str = "10,5,1,10,3,8,5,2,2";
const EXAMPLE2: &str =
    "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 29);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 781);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 3);
}
