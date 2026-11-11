use everybody_codes::event2026::quest13::*;

const EXAMPLE1: &str = "\
hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4";

const EXAMPLE2: &str = "\
hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4

hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:15

hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:20";

const EXAMPLE3: &str = "\
hands 6:14:20
direction CW:CW:CW
current 4:3:4
move 254374424007:244565274211:254769377653

hands 7:7:26
direction CW:CW:CCW
current 0:1:6
move 289073190139:300457172176:300334525277

hands 4:8:12
direction CW:CCW:CW
current 1:2:3
move 345074893841:358877889594:372680885348";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 5);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 21);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 436179922640);
}
