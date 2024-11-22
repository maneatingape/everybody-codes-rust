use everybody_codes::event2024::quest15::*;

const EXAMPLE1: &str = "\
#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########";

const EXAMPLE2: &str = "\
##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 26);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 38);
}

#[test]
fn part3_test() {
    // No test
}
