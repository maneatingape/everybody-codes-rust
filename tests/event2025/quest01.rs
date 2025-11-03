use everybody_codes::event2025::quest01::*;

const EXAMPLE: &str = "\
Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), "Fyrryn");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE), "Elarzris");
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE), "Fyrryn");
}
