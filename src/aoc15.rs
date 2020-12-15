use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day15.txt").unwrap();
    println!("15:1 {}", run_1(&input));
    println!("15:2 {}", run_2(&input));
}

#[derive(Debug, Clone)]
struct Spoken {
    turn: usize,
    cnt: usize,
}

fn solve(input: &str, break_at_turn: usize) -> usize {
    let (_, vals) = parse(input).unwrap();
    let mut last_spoken_lu = HashMap::new();

    for (t, v) in vals.iter().enumerate() {
        let turn = t + 1;
        last_spoken_lu.insert(*v, vec![turn]);
    }

    let mut turn = vals.len();
    let mut last_spoken = vals[vals.len() - 1];
    loop {
        if turn == break_at_turn {
            return last_spoken;
        }

        turn += 1;
        let s = last_spoken_lu.get_mut(&last_spoken).unwrap();
        if s.len() == 1 {
            // last time this was spoken, it was the first time
            last_spoken = 0;
        } else {
            // This word has been spoken before, diff between the last two turns
            last_spoken = s[s.len() - 1] - s[s.len() - 2];
        }
        //
        let e = last_spoken_lu.entry(last_spoken).or_insert(vec![]);
        e.push(turn);
        if e.len() > 2 {
            e.remove(0);
        }
    }
}

fn run_1(input: &str) -> usize {
    solve(input, 2020)
}

fn run_2(input: &str) -> usize {
    solve(input, 30_000_000)
}

fn parse(i: &str) -> nom::IResult<&str, Vec<usize>> {
    let (i, vals) =
        nom::multi::separated_list1(nom::character::complete::char(','), crate::helper::uval)(i)?;
    Ok((i, vals))
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc15_run_1() {
        assert_eq!(super::run_1("0,3,6"), 436);
        assert_eq!(super::run_1("1,3,2"), 1);
        assert_eq!(super::run_1("2,1,3"), 10);
        assert_eq!(super::run_1("1,2,3"), 27);
        assert_eq!(super::run_1("2,3,1"), 78);
        assert_eq!(super::run_1("3,2,1"), 438);
        assert_eq!(super::run_1("3,1,2"), 1836);
    }

    #[test]
    fn aoc15_run_2() {
        // Correct, but takes a long time to run
        // assert_eq!(super::run_2("0,3,6"), 175594);
        // assert_eq!(super::run_2("1,3,2"), 2578);
        // assert_eq!(super::run_2("2,1,3"), 3544142);
        // assert_eq!(super::run_2("1,2,3"), 261214);
        // assert_eq!(super::run_2("2,3,1"), 6895259);
        // assert_eq!(super::run_2("3,2,1"), 18);
        // assert_eq!(super::run_2("3,1,2"), 362);
    }
}
