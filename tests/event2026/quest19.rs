use everybody_codes::event2026::quest19::*;

const EXAMPLE1: &str = "\
CCCCC
MMMMM
YYYYY
KKKKK
CCMYY

CMYKC
CMYKC
CMYKC
CMYKC
CMYKC";

const EXAMPLE2: &str = "\
CCCCC
MMMMM
YYYYY
KKKKK
CCMYY

14231
14231
14231
14231
14231";

const EXAMPLE3: &str = "\
CCCCYMMM
CCMMMYYY
CCCYYYMM
MMCCCYYY
YYYMMMCC
CCYYMMKK
MMYYCCKK
YYYCCMMM";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 144);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 180);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 8000);
}
