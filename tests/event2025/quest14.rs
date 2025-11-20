use everybody_codes::event2025::quest14::*;

const EXAMPLE1: &str = "\
.#.##.
##..#.
..##.#
.#.##.
.###..
###.##";

const EXAMPLE2: &str = "\
#......#
..#..#..
.##..##.
...##...
...##...
.##..##.
..#..#..
#......#";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 200);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 39349);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), 278388552);
}
