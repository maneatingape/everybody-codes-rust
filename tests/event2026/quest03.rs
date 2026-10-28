use everybody_codes::event2026::quest03::*;

const EXAMPLE1: &str = "\
ISO-3210 7654
ISO-8742 2468
ISO-1111 1111";

const EXAMPLE2: &str = "\
ISO-3210 7654
ISO-8742 2468
ISO-1 1111
ISO-13 777
ISO-34 140856
ISO-789 2465112
ISO-2341 945";

const EXAMPLE3: &str = "\
ISO2-3 23
ISO2-61 149
ISO2-978 99
ISO2-5887 5923
ISO2-76988 123749
ISO2-435657 929977
ISO2-3949 870737";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 1478);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 1161463);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 50987);
}
