use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::recognize,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day7.txt").unwrap();
    println!("day7-1: {}", run_1(&input));
    println!("day7-2: {}", run_2(&input));
}

pub fn run_1(input: &str) -> usize {
    let (_, bags) = bags(input).unwrap();
    let bags: HashMap<&str, Vec<Content>> = bags.into_iter().collect();

    let mut can_contain: HashSet<String> = HashSet::new();
    let mut searched: HashSet<String> = HashSet::new();

    let mut to_search_for = vec!["shiny gold"];

    while !to_search_for.is_empty() {
        let mut new_to_search_for: Vec<&str> = Vec::new();

        for s in to_search_for {
            searched.insert(s.to_string());
            let bags = bags.iter().filter(|(_name, contents)| {
                contents
                    .iter()
                    .find(|Content { bag, .. }| bag == s)
                    .is_some()
            });
            bags.clone().for_each(|(name, _)| {
                can_contain.insert(name.to_string());
            });

            bags.filter(|(name, _)| !searched.contains(**name))
                .for_each(|(name, _)| new_to_search_for.push(&name));
        }

        to_search_for = new_to_search_for;
    }

    can_contain.len()
}

fn count_bags(
    name: &str,
    bags: &HashMap<&str, Vec<Content>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(v) = cache.get(name) {
        return *v;
    }

    let sub_bags = bags.get(name).unwrap();

    let count = 1 + sub_bags
        .iter()
        .map(|Content { bag, quantity }| quantity * count_bags(bag, bags, cache))
        .sum::<usize>();
    cache.insert(name.to_string(), count);
    count
}

pub fn run_2(input: &str) -> usize {
    let (_, bags) = bags(input).unwrap();
    let bags: HashMap<&str, Vec<Content>> = bags.into_iter().collect();
    let mut cache: HashMap<String, usize> = HashMap::new();

    let ans = count_bags("shiny gold", &bags, &mut cache);

    // subtract one as we will count the shiny gold bag in count_bags
    ans - 1
}

struct Content {
    bag: String,
    quantity: usize,
}

fn bag(i: &str) -> IResult<&str, &str> {
    recognize(separated_pair(alpha1, space1, alpha1))(i)
}

fn no_content(i: &str) -> IResult<&str, Vec<Content>> {
    let (i, _) = tag("no other bags")(i)?;
    Ok((i, Vec::new()))
}

fn bag_content(i: &str) -> IResult<&str, Content> {
    let (i, quantity) = crate::helper::uval::<usize>(i)?;
    let (i, _) = space1(i)?;
    let (i, bag) = bag(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = alt((tag("bags"), tag("bag")))(i)?;
    Ok((
        i,
        Content {
            bag: bag.to_string(),
            quantity,
        },
    ))
}

fn bag_contents(i: &str) -> IResult<&str, (&str, Vec<Content>)> {
    let (i, name) = bag(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = tag("bags contain")(i)?;
    let (i, _) = space1(i)?;
    let (i, content) = alt((separated_list1(tag(", "), bag_content), no_content))(i)?;
    let (i, _) = tag(".")(i)?;
    Ok((i, (name, content)))
}

fn bags(i: &str) -> IResult<&str, Vec<(&str, Vec<Content>)>> {
    separated_list1(newline, bag_contents)(i)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const INPUT_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn aoc7_parse() {
        let (i, bags) = super::bags(INPUT).unwrap();
        assert_eq!("", i);
        assert_eq!(bags.len(), 9);
        let bag = &bags[3];
        assert_eq!(bag.0, "muted yellow");
        let bag = &bags[8];
        assert_eq!(bag.0, "dotted black");
        assert!(bag.1.is_empty());
    }
    #[test]
    fn aoc7_run_1() {
        assert_eq!(super::run_1(INPUT), 4);
    }

    #[test]
    fn aoc7_run_2() {
        assert_eq!(super::run_2(INPUT), 32);
        assert_eq!(super::run_2(INPUT_2), 126);
    }
}
