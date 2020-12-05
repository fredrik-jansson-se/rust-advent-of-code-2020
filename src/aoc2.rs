use crate::helper;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Policy {
    min: usize,
    max: usize,
    c: char,
}

fn parse_policy(i: &str) -> IResult<&str, Policy> {
    let (i, min) = helper::uval(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, max) = helper::uval(i)?;
    let (i, _) = space1(i)?;
    let (i, c) = anychar(i)?;
    Ok((i, Policy { min, max, c }))
}

fn parse_passwords(i: &str) -> IResult<&str, Vec<(Policy, &str)>> {
    let password = separated_pair(|i| parse_policy(i), tag(": "), alpha1);
    separated_list1(newline, password)(i)
}

pub fn run() {
    let input = fs::read_to_string("day2.txt").unwrap();
    println!("2:1: {}", run_1(&input));
    println!("2:2: {}", run_2(&input));
}

fn is_valid_1(pwd: &str, policy: &Policy) -> bool {
    let mut freq: HashMap<char, usize> = HashMap::new();

    pwd.chars().for_each(|c| {
        let f = freq.entry(c).or_insert(0);
        *f += 1
    });

    let f = freq.get(&policy.c).unwrap_or(&0);
    *f >= policy.min && *f <= policy.max
}

fn run_1(input: &str) -> usize {
    let (_, passwords) = parse_passwords(input).unwrap();
    passwords
        .into_iter()
        .filter(|(pol, pwd)| is_valid_1(pwd, pol))
        .count()
}

fn is_valid_2(pwd: &str, policy: &Policy) -> bool {
    let c_string = policy.c.to_string();
    let range = (policy.min - 1)..(policy.min);
    let c1 = pwd.get(range).map(|c1| c1 == c_string);
    let range = (policy.max - 1)..(policy.max);
    let c2 = pwd.get(range).map(|c1| c1 == c_string);

    match (c1, c2) {
        (Some(true), None) | (Some(true), Some(false)) => true,
        (Some(false), Some(true)) | (None, Some(true)) => true,
        _ => false,
    }
}

fn run_2(input: &str) -> usize {
    let (_, passwords) = parse_passwords(input).unwrap();
    passwords
        .into_iter()
        .filter(|(pol, pwd)| is_valid_2(pwd, pol))
        .count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn aoc2_parse() {
        assert_eq!(
            super::parse_policy("1-3 a").unwrap().1,
            super::Policy {
                min: 1,
                max: 3,
                c: 'a'
            }
        );

        let (_, passwords) = super::parse_passwords(INPUT).unwrap();
        assert_eq!(passwords.len(), 3);
        assert_eq!(
            passwords[1].0,
            super::Policy {
                min: 1,
                max: 3,
                c: 'b'
            }
        );
        assert_eq!(passwords[1].1, "cdefg");
    }

    #[test]
    fn aoc2_run_1() {
        use super::*;
        assert_eq!(run_1(&INPUT), 2);
    }

    #[test]
    fn aoc2_run_2() {
        use super::*;
        assert_eq!(run_2(&INPUT), 1);
    }
}
