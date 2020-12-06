use std::fs;

#[derive(Debug)]
enum Cmd {
    Front,
    Back,
    Left,
    Right,
}

impl std::convert::From<char> for Cmd {
    fn from(c: char) -> Self {
        match c {
            'F' => Self::Front,
            'B' => Self::Back,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn parse_cmds(i: &str) -> Vec<Vec<Cmd>> {
    i.lines()
        .map(|line| {
            line.chars()
                .map(std::convert::Into::into)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn boarding_pass(p: &[Cmd]) -> (usize, usize) {
    let mut top_row = 128;
    let mut low_row = 0;
    assert_eq!(p.len(), 10);
    for cmd in &p[0..7] {
        // dbg! {(cmd, low_row, top_row)};
        match cmd {
            Cmd::Front => {
                top_row -= (top_row - low_row) / 2;
            }
            Cmd::Back => {
                low_row += (top_row - low_row) / 2;
            }
            _ => unreachable!(),
        }
    }
    let mut max_seat = 8;
    let mut min_seat = 0;
    for cmd in &p[7..10] {
        // dbg! {(cmd, min_seat, max_seat)};
        match cmd {
            Cmd::Right => {
                min_seat += (max_seat - min_seat) / 2;
            }
            Cmd::Left => {
                max_seat -= (max_seat - min_seat) / 2;
            }
            _ => unreachable!(),
        }
    }

    // dbg! {(low_row, min_seat)}
    (low_row, min_seat)
}

pub fn run() {
    let input = fs::read_to_string("day5.txt").unwrap();

    println!("5:1 {}", run_1(&input));
    println!("5:2 {}", run_2(&input));
}

fn seat_id((row, seat): (usize, usize)) -> usize {
    row * 8 + seat
}

fn run_1(input: &str) -> usize {
    let bps = parse_cmds(input);

    bps.iter()
        .map(|bp| boarding_pass(bp))
        .map(|rs| seat_id(rs))
        .max()
        .unwrap()
}

fn run_2(input: &str) -> usize {
    let bps = parse_cmds(input);

    let taken: Vec<usize> = bps
        .iter()
        .map(|bp| boarding_pass(bp))
        .map(|rs| seat_id(rs))
        .collect();
    let first_seat_id: usize = 1 * 8 + 0;
    let last_seat_id: usize = 126 * 8 + 7;
    (first_seat_id..last_seat_id)
        .find(|seat_id| {
            taken.contains(&(seat_id - 1))
                && !taken.contains(seat_id)
                && taken.contains(&(seat_id + 1))
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc5_run_1() {
        assert_eq!(super::run_1("FBFBBFFRLR"), 357);
        assert_eq!(super::run_1("BFFFBBFRRR"), 567);
        assert_eq!(super::run_1("FFFBBBFRRR"), 119);
        assert_eq!(super::run_1("BBFFBBFRLL"), 820);
    }
}
