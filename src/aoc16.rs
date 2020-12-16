use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, none_of},
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();

    println!("16:1 - {:?}", run_1(&input));
    println!("16:2 - {:?}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let (_, notes) = parse(input).unwrap();

    notes
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&value| {
            let is_valid = notes.field_ranges.iter().any(|(_, range)| {
                range
                    .iter()
                    .any(|(low, high)| value >= low && value <= high)
            });
            !is_valid
        })
        .sum()
}

fn solve_2(input: &str) -> HashMap<String, usize> {
    let (_, notes) = parse(input).unwrap();
    let valid_tickets = notes
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            let is_valid = ticket.iter().all(|value| {
                notes.field_ranges.iter().any(|(_, range)| {
                    range
                        .iter()
                        .any(|(low, high)| value >= low && value <= high)
                })
            });
            is_valid
        })
        .collect::<Vec<_>>();

    let mut validations: HashMap<String, Vec<FieldRange>> =
        notes.field_ranges.into_iter().collect();
    let my_ticket = notes.my_ticket;
    let mut name_to_index: HashMap<String, usize> = HashMap::new();

    let mut indices_to_check: VecDeque<usize> = (0..validations.len()).collect();

    while let Some(idx) = indices_to_check.pop_front() {
        // Search valid tickets for a validation that matches all
        // values at this idx
        let values_at_idx = valid_tickets
            .iter()
            .map(|vals| vals[idx])
            .collect::<Vec<_>>();

        // Find all possible validations for this index
        let mut valids: Vec<String> = validations
            .iter()
            .filter(|(_k, validations)| {
                // Do we have any validation that matches all the
                // values in values_at_idx
                values_at_idx.iter().all(|value| {
                    validations.iter().any(|(low, high)| {
                        // dbg! {(value, low, high)};
                        value >= low && value <= high
                    })
                })
            })
            .map(|(name, _)| name.to_string())
            .collect();
        // If a single matches, use that validation
        if valids.len() == 1 {
            let name = valids.remove(0);
            validations.remove(&name);
            name_to_index.insert(name, idx);
        } else {
            // Otherwise, recheck it later
            indices_to_check.push_back(idx);
        }
    }

    name_to_index
        .into_iter()
        .map(|(k, idx)| (k, my_ticket[idx]))
        .collect()
}

fn run_2(input: &str) -> usize {
    let fields = solve_2(input);

    fields
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(*v)
            } else {
                None
            }
        })
        .fold(1, |acc, v| acc * v)
}

type FieldRange = (usize, usize);

struct Notes {
    field_ranges: Vec<(String, Vec<FieldRange>)>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

fn parse_fieldrange(i: &str) -> IResult<&str, (String, Vec<FieldRange>)> {
    let range = separated_pair(crate::helper::uval, tag("-"), crate::helper::uval);
    let (i, name) = recognize(many1(none_of(":")))(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, ranges) = separated_list1(tag(" or "), range)(i)?;
    Ok((i, (name.to_string(), ranges)))
}

fn parse(i: &str) -> IResult<&str, Notes> {
    let (i, field_ranges) = separated_list1(newline, parse_fieldrange)(i)?;
    let (i, _) = many1(newline)(i)?;

    let (i, _) = tag("your ticket:")(i)?;
    let (i, _) = newline(i)?;
    let (i, my_ticket) = separated_list1(tag(","), crate::helper::uval)(i)?;
    let (i, _) = many1(newline)(i)?;

    let (i, _) = tag("nearby tickets:")(i)?;
    let (i, _) = newline(i)?;
    let (i, nearby_tickets) =
        separated_list1(newline, separated_list1(tag(","), crate::helper::uval))(i)?;
    Ok((
        i,
        Notes {
            field_ranges,
            my_ticket,
            nearby_tickets,
        },
    ))
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn aoc16_parse() {
        let (_, (name, rng)) = super::parse_fieldrange("class: 1-3 or 5-7").unwrap();
        assert_eq!(name, "class");
        assert_eq!(rng.len(), 2);

        let (_, (name, rng)) =
            super::parse_fieldrange("departure location: 25-568 or 594-957 or 1-2").unwrap();
        assert_eq!(name, "departure location");
        assert_eq!(rng.len(), 3);

        let (_, notes) = super::parse(INPUT_1).unwrap();
        assert_eq!(notes.field_ranges.len(), 3);
        assert_eq!(notes.my_ticket.len(), 3);
        assert_eq!(notes.nearby_tickets.len(), 4);
    }

    #[test]
    fn aoc16_run_1() {
        assert_eq!(super::run_1(INPUT_1), 71);
    }
    const INPUT_2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn aoc16_run_2() {
        let fields = super::solve_2(INPUT_2);
        assert_eq!(fields.get("row").unwrap(), &11);
        assert_eq!(fields.get("class").unwrap(), &12);
        assert_eq!(fields.get("seat").unwrap(), &13);
        //
    }
}
