use everybody_codes::event2024::quest02::*;

const EXAMPLE1: &str = "\
WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";

const EXAMPLE2: &str = "\
WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END";

const EXAMPLE3: &str = "\
WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 37);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 10);
}
