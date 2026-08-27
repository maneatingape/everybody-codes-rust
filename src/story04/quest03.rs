use crate::util::parse::*;

pub fn part1(notes: &str) -> usize {
    solve(notes).into_iter().sum()
}

pub fn part2(notes: &str) -> usize {
    solve(notes).into_iter().max().unwrap()
}

pub fn part3(notes: &str) -> usize {
    solve(notes).into_iter().max().unwrap()
}

fn solve(notes: &str) -> [usize; 2] {
    let tokens: Vec<_> = notes.lines().flat_map(|line| line.split('=')).collect();
    let double = |s: &str| s.repeat(2).bytes().map(|b| usize::from(b - b'0')).collect();

    let width: usize = tokens[1].unsigned();
    let height: usize = tokens[3].unsigned();
    let horizontal: Vec<_> = double(tokens[5]);
    let vertical: Vec<_> = double(tokens[7]);

    let rows = horizontal.len();
    let columns = vertical.len();
    let mut row_parity = 0;
    let mut total = [0; 2];

    for y in 0..rows.min(height) {
        row_parity ^= usize::from(y > 0 && horizontal[y] == 0);
        let mut column_parity = 0;

        if horizontal[y] == horizontal[(y + 1) % rows] {
            for x in 0..columns.min(width) {
                column_parity ^= usize::from(x > 0 && vertical[x] == y % 2);

                if horizontal[y] == x % 2
                    && vertical[x] == y % 2
                    && vertical[(x + 1) % columns] == y % 2
                {
                    let repeat_x = (width - x).div_ceil(columns);
                    let repeat_y = (height - y).div_ceil(rows);
                    total[row_parity ^ column_parity] += repeat_x * repeat_y;
                }
            }
        }
    }

    total
}
