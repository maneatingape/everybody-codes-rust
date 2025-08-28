use everybody_codes::story02::quest02::*;

const EXAMPLE1: &str = "GRBGGGBBBRRRRRRRR";
const EXAMPLE2: &str = "BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";
const EXAMPLE3: &str = "BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 2955);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 2953681);
}
