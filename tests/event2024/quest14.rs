use everybody_codes::event2024::quest14::*;

const EXAMPLE1: &str = "\
U5,R3,D2,L5,U4,R5,D2";

const EXAMPLE2: &str = "\
U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1";

const EXAMPLE3: &str = "\
U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 32);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 46);
}
