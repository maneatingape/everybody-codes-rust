use everybody_codes::event2024::quest16::*;

const EXAMPLE1: &str = "\
1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>";

const EXAMPLE2: &str = "\
1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), ">.- -.- ^,-");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 280014668134);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE2), "627 128");
}
