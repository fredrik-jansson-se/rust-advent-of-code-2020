use std::fs;

use super::helper::*;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

pub fn run() {
    let input = fs::read_to_string("day13.txt").unwrap();
    println!("13:1 {}", run_1(&input));
    println!("13:2 {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let (_, notes) = parse(input).unwrap();
    let mults = notes
        .buses
        .iter()
        .filter_map(|bus| *bus)
        .map(|bus| (bus, (notes.earliest_at as f32) / (bus as f32)))
        .map(|(bus, log)| (bus, log.ceil() as usize))
        .collect::<Vec<_>>();
    let wait_time = mults
        .iter()
        .map(|(bus, mult)| (bus, bus * mult - notes.earliest_at))
        .min_by(|(_, t), (_, t2)| t.cmp(t2))
        .unwrap();
    // Multiplys bus id with wait_time
    wait_time.0 * wait_time.1
}

fn run_2(input: &str) -> u128 {
    let (_, notes) = parse(input).unwrap();
    let mut buses = notes
        .buses
        .iter()
        .enumerate()
        .filter_map(|(idx, bus)| bus.map(|bus| (bus as i128, idx as i128)))
        .collect::<Vec<_>>();
    buses.sort_by(|(b1, _), (b2, _)| b2.cmp(b1));

    use std::collections::HashMap;
    let mods: HashMap<i128, i128> = buses
        .iter()
        .map(|(bus, idx)| (*bus, -idx % bus))
        .map(|(bus, idx)| (bus, if idx < 0 { bus + idx } else { idx }))
        .collect::<HashMap<_, _>>();

    let mut r = buses[0].0;
    let mut val = *mods.get(&r).unwrap();

    for (b, _) in &buses[1..] {
        let m = *mods.get(b).unwrap();
        while (val % *b) != m {
            // dbg! {(r,val)};
            val += r;
        }
        r *= b;
    }

    val as u128
}

#[derive(Debug)]
struct Notes {
    earliest_at: usize,
    buses: Vec<Option<usize>>,
}

fn bus(i: &str) -> IResult<&str, Option<usize>> {
    alt((map(char('x'), |_| None), map(uval, |v| Some(v))))(i)
}

fn buses(i: &str) -> IResult<&str, Vec<Option<usize>>> {
    separated_list1(char(','), bus)(i)
}

fn parse(i: &str) -> IResult<&str, Notes> {
    let (i, earliest_at) = uval(i)?;
    let (i, _) = newline(i)?;
    let (i, buses) = buses(i)?;
    Ok((i, Notes { earliest_at, buses }))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn aoc13_1() {
        assert_eq!(super::run_1(INPUT), 295);
    }

    #[test]
    fn aoc13_2() {
        let input = "0\n7,13,x,x,59,x,31,19";
        assert_eq!(super::run_2(input), 1068781);
        let input = "0\n17,x,13,19";
        assert_eq!(super::run_2(input), 3417);
        let input = "0\n67,7,59,61";
        assert_eq!(super::run_2(input), 754018);
        let input = "0\n67,x,7,59,61";
        assert_eq!(super::run_2(input), 779210);
        let input = "0\n67,7,x,59,61";
        assert_eq!(super::run_2(input), 1261476);
        let input = "0\n1789,37,47,1889";
        assert_eq!(super::run_2(input), 1202161486);
    }
}
