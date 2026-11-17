use everybody_codes::event2026::quest17::*;

const EXAMPLE1: &str = "\
####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##";

const EXAMPLE2: &str = "\
####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##";

const EXAMPLE3: &str = "\
####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 21);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 94);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 31);
}
