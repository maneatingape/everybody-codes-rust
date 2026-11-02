use everybody_codes::event2026::quest06::*;

const EXAMPLE1: &str = "\
BRDS:FLWR,SLTT,YSTS
FLWR:SLTT,YSTS
SLTT:YSTS,MLKK,BTTR
NULL:BRDS,FLWR,SLTT
YSTS:BRDS,FLWR
MLKK:SPCS,NULL
HNYN:BRDS,FLWR,SLTT,YSTS";

const EXAMPLE2: &str = "\
MEAL:BRDS,CAKE,DOUG
BRDS:DOUG,FLWR
CAKE:DOUG,FLWR,NULLv2,NULLv3
DOUG:FLWR,YSTS,SLTT,NULLv1
FLWR:BRDS,DOUG
YSTS:SPCS
SOUP:BRDS,CAKE,DOUG,FLWR";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 6);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 3);
}
