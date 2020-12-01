use super::helper::*;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::*;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day1.txt").unwrap();
    //47424
    println!("1:1: {}", run_1(&input));
    println!("1:2: {}", run_2(&input));
}

fn parse_input(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(newline, usize_val)(i)
}

fn solve(inputs: &HashSet<usize>, target: usize) -> Option<usize> {
    inputs
        .iter()
        .filter(|&&v| v <= target)
        .find(|&v| inputs.contains(&(target - *v)))
        .map(|&val| val * (target - val))
}

fn run_1(input: &str) -> usize {
    let (_, input) = parse_input(input).unwrap();
    let input: std::collections::HashSet<usize> = input.into_iter().collect();
    solve(&input, 2020).unwrap()
}

fn run_2(input: &str) -> usize {
    let (_, input) = parse_input(input).unwrap();
    let input: std::collections::HashSet<usize> = input.into_iter().collect();
    input
        .iter()
        .find_map(|v1| solve(&input, 2020 - v1).map(|v2| v1 * v2))
        .unwrap()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn aoc1_run_1() {
        assert_eq!(super::run_1(INPUT), 514579);
    }

    #[test]
    fn aoc1_run_2() {
        use super::*;
        assert_eq!(super::run_2(INPUT), 241861950);
    }
}
