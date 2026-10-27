use everybody_codes::event2026::quest02::*;

const EXAMPLE1: &str = "\
4<3 2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
4>1 2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 2<4 3>2 3<4 1>2 3>5 1<3 4>5";

const EXAMPLE2: &str = "\
2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 3>2 3<4 1>2 3>5 1<3 4>5";

const EXAMPLE3: &str = "\
3>1 1>2 1<5 4<2 3>5
2>1 2<5 3>1 4>3 2<4 5<3
5>1 3<4 1>2 3>5";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 122157);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 122157);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 122157);
}
