use everybody_codes::story04::quest02::*;

const EXAMPLE1: &str = "\
START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]
MOVES=ABCCBABCA";

const EXAMPLE2: &str = "\
START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]";

const EXAMPLE3: &str = "\
START=[0,0]
A=[0,0]
B=[80,15]
C=[5,30]";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 8);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 25);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 42);
    assert_eq!(part3(EXAMPLE3), 432);
}
