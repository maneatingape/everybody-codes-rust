use everybody_codes::event2026::quest08::*;

const EXAMPLE1: &str = "\
....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8....";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 32);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 187);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE1), 899889);
}
