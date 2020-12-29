use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day21.txt").unwrap();
    println!("21:1 {:}", run_1(&input));
    println!("21:2 {:?}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let (_, foods) = parse(input).unwrap();

    let mut all_ingredients_count: HashMap<&str, usize> = HashMap::new();

    let mut alergens_to_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, alergens) in foods.iter() {
        // Insert count of the ingredients
        ingredients.iter().for_each(|ing| {
            *all_ingredients_count.entry(*ing).or_insert(0) += 1;
        });

        for alergen in alergens {
            if let Some(previous) = alergens_to_ingredients.get_mut(alergen) {
                let in_both: HashSet<&str> =
                    previous.intersection(ingredients).map(|v| *v).collect();
                *previous = in_both;
            } else {
                alergens_to_ingredients.insert(alergen.clone(), ingredients.clone());
            }
        }
    }

    // Filter out all ingredients that has know alergens
    all_ingredients_count
        .iter()
        .filter(|(ing, cnt)| {
            !alergens_to_ingredients
                .iter()
                .any(|(_algergen, ingredients)| ingredients.contains(*ing))
        })
        .map(|(_ing, c)| c)
        .sum()
}

fn run_2(input: &str) -> String {
    let (_, foods) = parse(input).unwrap();

    let mut alergens_to_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, alergens) in foods.iter() {
        for alergen in alergens {
            if let Some(previous) = alergens_to_ingredients.get_mut(alergen) {
                let in_both: HashSet<&str> =
                    previous.intersection(ingredients).map(|v| *v).collect();
                *previous = in_both;
            } else {
                alergens_to_ingredients.insert(alergen.clone(), ingredients.clone());
            }
        }
    }

    let mut new_alergens_to_ingredients: HashMap<&str, &str> = HashMap::new();
    while !alergens_to_ingredients.is_empty() {
        let (one_ingredients, mut multi_ingredients): (
            Vec<(&str, HashSet<&str>)>,
            Vec<(&str, HashSet<&str>)>,
        ) = alergens_to_ingredients
            .into_iter()
            .partition(|(_, ing)| ing.len() == 1);

        for (a, i) in one_ingredients {
            let ing = *i.iter().next().unwrap();
            new_alergens_to_ingredients.insert(a, ing);
            for (_, ings) in multi_ingredients.iter_mut() {
                ings.remove(ing);
            }
        }
        alergens_to_ingredients = multi_ingredients.into_iter().collect();
    }

    let mut sorted_alergens: Vec<&str> = new_alergens_to_ingredients.keys().map(|k| *k).collect();
    sorted_alergens.sort();

    sorted_alergens
        .into_iter()
        .map(|a| *new_alergens_to_ingredients.get(a).unwrap())
        .collect::<Vec<&str>>()
        .join(",")
}

fn parse_food(i: &str) -> IResult<&str, (HashSet<&str>, Vec<&str>)> {
    let (i, ingredients) = separated_list1(space1, alpha1)(i)?;
    let (i, _) = tag(" (contains ")(i)?;
    let (i, alergens) = separated_list1(tag(", "), alpha1)(i)?;
    let (i, _) = tag(")")(i)?;
    Ok((i, (ingredients.into_iter().collect(), alergens)))
}

fn parse(i: &str) -> IResult<&str, Vec<(HashSet<&str>, Vec<&str>)>> {
    separated_list1(newline, parse_food)(i)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn aoc21_parse() {
        let (i, foods) = super::parse(INPUT_1).unwrap();
        assert_eq!(foods.len(), 4);
        assert_eq!(foods[0].0.len(), 4);
        assert_eq!(foods[0].1.len(), 2);
    }
    #[test]
    fn aoc21_run_1() {
        assert_eq!(super::run_1(INPUT_1), 5);
    }
    #[test]
    fn aoc21_run_2() {
        assert_eq!(&super::run_2(INPUT_1), "mxmxvkd,sqjhc,fvjkl");
    }
}
