use everybody_codes::event2026::quest18::*;

const EXAMPLE1: &str = "\
x=0 y=5
x=5 y=7
x=6 y=8
x=7 y=1
x=8 y=3
x=10 y=6
x=15 y=3
x=18 y=4";

const EXAMPLE2: &str = "\
x=0 y=5 width=1
x=1 y=4 width=3
x=5 y=3 width=2
x=7 y=9 width=1
x=8 y=8 width=1
x=9 y=9 width=3
x=12 y=3 width=1
x=16 y=7 width=1
x=19 y=3 width=5
x=25 y=5 width=1
x=29 y=1 width=2
x=31 y=6 width=1";

const EXAMPLE3: &str = "\
x=0 y=5000 width=1
x=31000000 y=6000 width=1";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 96);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 205);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 155000006000);
}
