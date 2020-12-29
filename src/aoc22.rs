use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{many_m_n, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day22.txt").unwrap();
    println!("22:1 {}", run_1(&input));
    println!("22:2 {}", run_2(&input));
}

fn parse_player(i: &str) -> IResult<&str, Vec<usize>> {
    let re = regex::Regex::new(r#"(Player \d:)"#).unwrap();
    let (i, _) = nom::regexp::str::re_capture(re)(i)?;
    let (i, _) = newline(i)?;
    separated_list1(newline, crate::helper::uval)(i)
}

fn parse(i: &str) -> IResult<&str, (VecDeque<usize>, VecDeque<usize>)> {
    let (i, (p1, p2)) = separated_pair(parse_player, many_m_n(2, 2, newline), parse_player)(i)?;
    Ok((i, (p1.into_iter().collect(), p2.into_iter().collect())))
}

fn run_1(input: &str) -> usize {
    let (_, (mut p1, mut p2)) = parse(input).unwrap();

    while !p1.is_empty() && !p2.is_empty() {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card > p2_card {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }

    let winner = if p1.is_empty() { p2 } else { p1 };
    let mut winner = winner.into_iter().collect::<Vec<_>>();
    winner.reverse();

    winner.into_iter().zip(1..).map(|(a, b)| a * b).sum()
}

fn run_2(input: &str) -> usize {
    let (_, (mut p1, mut p2)) = parse(input).unwrap();
    let mut game = 1;

    loop {
        let mut p1_won = false;
        let mut p1_previous_hands = HashSet::new();
        let mut p2_previous_hands = HashSet::new();
        println!("=== Game {} ===\n", game);
        for round in 1.. {
            println!("-- Round {} (Game {}) --\n", round, game);
            println!("Player 1's deck: {:?}", p1);
            println!("Player 2's deck: {:?}", p2);
            if p1_previous_hands.contains(&p1) || p2_previous_hands.contains(&p2) {
                p1_won = true;
                break;
            }

            p1_previous_hands.insert(p1.clone());
            p2_previous_hands.insert(p2.clone());
        }
        break;
    }
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn aoc22_run_1() {
        assert_eq!(super::run_1(INPUT_1), 306);
    }

    const INPUT_2: &str = "Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn aoc22_run_2() {
        // assert_eq!(super::run_2(INPUT_1), 291);
    }
}
