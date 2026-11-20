use crate::util::iter::*;

pub fn part1(notes: &str) -> u32 {
    solve(notes, |line| is_correct(line, 10))
}

pub fn part2(notes: &str) -> String {
    base11(solve(notes, |line| is_correct(line, 11)))
}

pub fn part3(notes: &str) -> String {
    base11(solve(notes, can_move_one))
}

fn solve(notes: &str, valid: fn(&str) -> bool) -> u32 {
    notes.lines().filter(|line| valid(line)).flat_map(str::bytes).map(segments).sum()
}

fn can_move_one(equation: &str) -> bool {
    let valid = |next: String| is_correct(&next, 11);

    is_correct(equation, 11)
        || moves(equation, change_one).any(valid)
        || moves(equation, remove_one).any(|next| moves(&next, attach_one).any(valid))
}

fn moves(equation: &str, lookup: fn(u8) -> &'static str) -> impl Iterator<Item = String> {
    equation.bytes().enumerate().flat_map(move |(i, b)| {
        let (prefix, suffix) = (&equation[..i], &equation[i + 1..]);
        lookup(b).chars().map(move |c| format!("{prefix}{c}{suffix}"))
    })
}

fn change_one(b: u8) -> &'static str {
    match b {
        b'0' => "69A",
        b'2' | b'5' => "3",
        b'3' => "25",
        b'6' => "09A",
        b'9' => "06A",
        b'A' => "069",
        _ => "",
    }
}

fn remove_one(b: u8) -> &'static str {
    match b {
        b'6' => "5",
        b'7' => "1",
        b'8' => "069A",
        b'9' => "35",
        b'+' => "-",
        _ => "",
    }
}

fn attach_one(b: u8) -> &'static str {
    match b {
        b'0' | b'6' | b'9' | b'A' => "8",
        b'1' => "7",
        b'3' => "9",
        b'5' => "69",
        b'-' => "+",
        _ => "",
    }
}

fn segments(b: u8) -> u32 {
    match b {
        b'0' | b'6' | b'9' | b'A' => 6,
        b'1' | b'+' | b'=' => 2,
        b'2' | b'3' | b'5' => 5,
        b'4' => 4,
        b'7' => 3,
        b'8' => 7,
        b'-' => 1,
        b' ' => 0,
        _ => unreachable!(),
    }
}

fn is_correct(equation: &str, radix: u32) -> bool {
    let [a, b, c, _, d, e, f] = equation.split_ascii_whitespace().chunk::<7>().next().unwrap();
    let parse = |s| i32::from_str_radix(s, radix).unwrap();
    let eval = |x, op, y| if op == "+" { parse(x) + parse(y) } else { parse(x) - parse(y) };

    eval(a, b, c) == eval(d, e, f)
}

fn base11(n: u32) -> String {
    let digit = char::from_digit(n % 11, 11).unwrap().to_uppercase();
    if n < 11 { digit.to_string() } else { format!("{}{digit}", base11(n / 11)) }
}
