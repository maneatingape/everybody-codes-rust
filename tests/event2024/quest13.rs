use everybody_codes::event2024::quest13::*;

const EXAMPLE1: &str = "\
#######
#6769##
S50505E
#97434#
#######";

const EXAMPLE2: &str = "\
SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 28);
}

#[test]
fn part2_test() {
    // No test
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 14);
}
