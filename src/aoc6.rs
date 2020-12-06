use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day6.txt").unwrap();
    println!("6:1: {}", run_1(&input));
    println!("6:2: {}", run_2(&input));
}

type Person = HashSet<char>;
type Group = Vec<Person>;

fn person(i: &str) -> IResult<&str, HashSet<char>> {
    let (i, v) = alpha1(i)?;
    Ok((i, v.chars().into_iter().collect()))
}

fn group(i: &str) -> IResult<&str, Vec<Person>> {
    separated_list1(newline, person)(i)
}

fn groups(i: &str) -> IResult<&str, Vec<Group>> {
    separated_list1(tag("\n\n"), group)(i)
}

fn run_1(input: &str) -> usize {
    let (_, groups) = groups(input).unwrap();

    // fold each group into its own
    groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold(HashSet::new(), |totals, person| {
                    totals.union(&person).map(|c| *c).collect()
                })
                .len()
        })
        .sum()
}

fn run_2(input: &str) -> usize {
    let (_, groups) = groups(input).unwrap();

    let all_possibles: HashSet<char> = ('a'..='z').collect();

    // fold each group into its own
    groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold(all_possibles.clone(), |totals, person| {
                    totals.intersection(&person).map(|c| *c).collect()
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    #[test]
    fn aoc6_parse() {
        let (i, groups) = super::groups(INPUT).unwrap();
        dbg!(i);
        assert!(i.is_empty());
        assert_eq!(groups.len(), 5);
    }

    #[test]
    fn aoc6_run_1() {
        assert_eq!(super::run_1(INPUT), 11);
    }

    #[test]
    fn aoc6_run_2() {
        assert_eq!(super::run_2(INPUT), 6);
    }
}
