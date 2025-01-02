use everybody_codes::event2024::quest20::*;

const EXAMPLE1: &str = "\
#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#";

const EXAMPLE2: &str = "\
####S####
#-.+++.-#
#.+.+.+.#
#-.+.+.-#
#A+.-.+C#
#.+-.-+.#
#.+.B.+.#
#########";

const EXAMPLE3: &str = "\
#......S......#
#-...+...-...+#
#.............#
#..+...-...+..#
#.............#
#-...-...+...-#
#.............#
#..#...+...+..#";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 1045);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 24);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 768790);
}
