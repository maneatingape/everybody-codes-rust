use everybody_codes::story04::quest01::*;

const EXAMPLE1: &str = "\
1,1,1,1,1
5,1,2,3,4,5,1,2,3,4
2,1,1,2,1,1,2,1,1,2,1,1
5,1,2,1,2,7,1,2,1,2,7,1,2,1,2";

const EXAMPLE2: &str = "\
5,3,1,1
5,3,1,1,5,1,1,3,4,8,1,1
5,3,1,1,5,1,1,3,4,8,2,1
10,9,9,8,8,7,7,6,6,5,5,4,4,3,3,2,2,1";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 34);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 43);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE1), 27);
    assert_eq!(part3(EXAMPLE2), 35);
}
