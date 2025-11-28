use everybody_codes::event2025::quest20::*;

const EXAMPLE1: &str = "\
T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....";

const EXAMPLE2: &str = "\
TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........";

const EXAMPLE3: &str = "\
T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S.........";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 32);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 23);
}
