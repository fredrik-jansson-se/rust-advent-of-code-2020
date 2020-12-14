use crate::helper::uval;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    combinator::{opt, recognize},
    multi::{many1, separated_list1},
    IResult,
};

use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day14.txt").unwrap();
    println!("14:1 {}", run_1(&input));
    println!("14:2 {}", run_2(&input));
}

fn run_1(input: &str) -> u64 {
    let (_, programs) = parse_1(input).unwrap();

    let mut mem = HashMap::new();

    for program in &programs {
        for (addr, val) in &program.mem_set {
            let mem_loc = mem.entry(addr).or_insert(0);
            *mem_loc = *val;
            *mem_loc |= program.or_mask;
            *mem_loc &= program.and_mask;
        }
    }

    mem.iter().fold(0, |acc, (_, v)| acc + v)
}

fn run_2(input: &str) -> u64 {
    let (_, mut programs) = parse_2(input).unwrap();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    for program in programs.iter_mut() {
        for (address, value) in program.mem_set.iter_mut() {
            // First set all bits that should be one
            for (i, m) in program.mask.iter().enumerate() {
                let bit = program.mask.len() - 1 - i;
                match m {
                    Some(1) => {
                        *address |= 1 << bit;
                    }
                    _ => (),
                }
            }

            let mut addresses: Vec<u64> = vec![*address];

            for (i, m) in program.mask.iter().enumerate() {
                let bit = program.mask.len() - 1 - i;
                if m.is_none() {
                    // Set bit to one
                    addresses.iter_mut().for_each(|addr| *addr |= 1 << bit);
                    let new_addresses = addresses
                        .iter()
                        .map(|addr| addr & !(1 << bit))
                        .collect::<Vec<_>>();
                    addresses.extend(new_addresses);
                }
            }

            addresses.iter().for_each(|addr| {
                *mem.entry(*addr).or_insert(0) = *value;
            })
        }
    }

    mem.iter().fold(0, |acc, (_, v)| acc + v)
}

fn parse_mask_1(i: &str) -> IResult<&str, (u64, u64)> {
    let (i, _) = tag("mask = ")(i)?;
    let (i, mask) = recognize(many1(one_of("X10")))(i)?;
    let mut or_mask = 0;
    let mut and_mask = 0;
    for c in mask.chars() {
        or_mask <<= 1;
        and_mask <<= 1;
        match c {
            '1' => {
                or_mask |= 1;
                and_mask |= 1;
            }
            '0' => {
                and_mask += 0;
            }
            _ => {
                and_mask |= 1;
            }
        }
    }
    Ok((i, (or_mask, and_mask)))
}

fn parse_mem(i: &str) -> IResult<&str, (u64, u64)> {
    let (i, _) = tag("mem[")(i)?;
    let (i, addr) = uval(i)?;
    let (i, _) = tag("] = ")(i)?;
    let (i, val) = uval(i)?;
    Ok((i, (addr, val)))
}

#[derive(Debug)]
struct Program1 {
    or_mask: u64,
    and_mask: u64,
    mem_set: Vec<(u64, u64)>,
}

fn parse_program_1(i: &str) -> IResult<&str, Program1> {
    let (i, (or_mask, and_mask)) = parse_mask_1(i)?;
    let (i, _) = newline(i)?;
    let (i, mems) = separated_list1(newline, parse_mem)(i)?;
    let (i, _) = opt(newline)(i)?;
    Ok((
        i,
        Program1 {
            or_mask,
            and_mask,
            mem_set: mems,
        },
    ))
}

fn parse_1(i: &str) -> IResult<&str, Vec<Program1>> {
    many1(parse_program_1)(i)
}

#[derive(Debug)]
struct Program2 {
    mask: Vec<Option<u8>>,
    mem_set: Vec<(u64, u64)>,
}

fn parse_mask_2(i: &str) -> IResult<&str, Vec<Option<u8>>> {
    let (i, _) = tag("mask = ")(i)?;
    let (i, mask) = recognize(many1(one_of("X10")))(i)?;
    let mut mask_vec = Vec::new();
    for c in mask.chars() {
        match c {
            '1' => {
                mask_vec.push(Some(1));
            }
            '0' => {
                mask_vec.push(Some(0));
            }
            _ => {
                mask_vec.push(None);
            }
        }
    }
    Ok((i, mask_vec))
}

fn parse_program_2(i: &str) -> IResult<&str, Program2> {
    let (i, mask) = parse_mask_2(i)?;
    let (i, _) = newline(i)?;
    let (i, mems) = separated_list1(newline, parse_mem)(i)?;
    let (i, _) = opt(newline)(i)?;
    Ok((
        i,
        Program2 {
            mask,
            mem_set: mems,
        },
    ))
}

fn parse_2(i: &str) -> IResult<&str, Vec<Program2>> {
    many1(parse_program_2)(i)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    const INPUT_2: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn aoc14_parse() {
        let (_, (or_mask, and_mask)) =
            super::parse_mask_1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(or_mask, 64);
        assert_eq!(and_mask, 68719476733);
        let (_, programs) = super::parse_1(INPUT_2).unwrap();
        assert_eq!(programs.len(), 2);
        let (_, programs) = super::parse_2(INPUT_2).unwrap();
        assert_eq!(programs.len(), 2);
    }

    #[test]
    fn aoc14_run_1() {
        assert_eq!(super::run_1(INPUT), 165);
    }

    #[test]
    fn aoc14_run_2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(super::run_2(input), 208);
    }
}
