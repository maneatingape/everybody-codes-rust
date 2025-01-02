use everybody_codes::event2024::quest10::*;

const EXAMPLE1: &str = "\
**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**";

const EXAMPLE2: &str = "\
**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), "PTBVRCZHFLJWGMNS");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 1851);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 3889);
}
