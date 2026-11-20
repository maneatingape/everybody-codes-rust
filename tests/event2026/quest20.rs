use everybody_codes::event2026::quest20::*;

const EXAMPLE1: &str = "\
5 - 4 = 9 - 8
2 + 1 = 3 + 0
6 + 7 = 4 + 9
1 + 2 = 3 + 4";

const EXAMPLE2: &str = "\
10 + 2A = 13 + 27
98 - 54 = 76 - 32
10 + 12 = 30 - 9
20 - 10 = 5 + 5";

const EXAMPLE3: &str = "\
4469 - 5934 = 1631 - 2AA7
8471 - 2276 = 4510 + 2736
A767 - 6104 = 1825 + 2538
9961 + 1140 = 5963 + 2974
9188 - 406A = 3598 + 1533
96A6 - 1606 = 4676 - 3524
9114 - 15A2 = 5258 + 2371
7023 - 2209 = 1433 + 3436
921A - 2696 = 2A91 + 3688
8492 - 2055 = 4186 + 3260
1234 + 4567 = 8901 + 2345";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 75);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), "101");
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), "693");
}
