use everybody_codes::event2025::quest10::*;

const EXAMPLE1: &str = "\
...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..";

const EXAMPLE2: &str = "\
...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..";

const EXAMPLE3_1: &str = "\
SSS
..#
#.#
#D.";

const EXAMPLE3_2: &str = "\
SSS
..#
..#
.##
.D#";

const EXAMPLE3_3: &str = "\
..S..
.....
..#..
.....
..D..";

const EXAMPLE3_4: &str = "\
.SS.S
#...#
...#.
##..#
.####
##D.#";

const EXAMPLE3_5: &str = "\
SSS.S
.....
#.#.#
.#.#.
#.D.#";

#[test]
fn part1_test() {
    assert_eq!(part1_testable(EXAMPLE1, 3), 27);
}

#[test]
fn part2_test() {
    assert_eq!(part2_testable(EXAMPLE2, 3), 27);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3_1), 15);
    assert_eq!(part3(EXAMPLE3_2), 8);
    assert_eq!(part3(EXAMPLE3_3), 44);
    assert_eq!(part3(EXAMPLE3_4), 4406);
    assert_eq!(part3(EXAMPLE3_5), 13033988838);
}
