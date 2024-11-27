use everybody_codes::event2024::quest18::*;

const EXAMPLE1: &str = "\
##########
..#......#
#.P.####P#
#.#...P#.#
##########";

const EXAMPLE2: &str = "\
#######################
...P..P...#P....#.....#
#.#######.#.#.#.#####.#
#.....#...#P#.#..P....#
#.#####.#####.#########
#...P....P.P.P.....P#.#
#.#######.#####.#.#.#.#
#...#.....#P...P#.#....
#######################";

const EXAMPLE3: &str = "\
##########
#.#......#
#.P.####P#
#.#...P#.#
##########";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 11);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 21);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 12);
}
