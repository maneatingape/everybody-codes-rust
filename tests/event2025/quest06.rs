use everybody_codes::event2025::quest06::*;

const EXAMPLE1: &str = "ABabACacBCbca";
const EXAMPLE2: &str = "AABCBABCABCabcabcABCCBAACBCa";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 11);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 3442321);
}
