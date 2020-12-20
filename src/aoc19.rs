use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day19.txt").unwrap();
    println!("19:1 - {}", run_1(&input));
    println!("19:2 - {}", run_2(&input));
}

#[derive(Debug)]
enum Rule {
    SubRule(Vec<Vec<usize>>),
    Str(String),
}

fn parse_sub_rule(i: &str) -> nom::IResult<&str, (usize, Rule)> {
    let (i, rule_num) = crate::helper::uval(i)?;
    let (i, _) = nom::bytes::complete::tag(": ")(i)?;
    let inner = nom::multi::separated_list1(nom::character::complete::space1, crate::helper::uval);
    let (i, sub_rules) = nom::multi::separated_list1(nom::bytes::complete::tag(" | "), inner)(i)?;
    Ok((i, (rule_num, Rule::SubRule(sub_rules))))
}

fn parse_str_rule(i: &str) -> nom::IResult<&str, (usize, Rule)> {
    let (i, rule_num) = crate::helper::uval(i)?;
    let (i, _) = nom::bytes::complete::tag(": ")(i)?;
    let (i, s) = nom::sequence::preceded(
        nom::bytes::complete::tag("\""),
        nom::sequence::terminated(
            nom::character::complete::alpha1,
            nom::bytes::complete::tag("\""),
        ),
    )(i)?;
    Ok((i, (rule_num, Rule::Str(s.to_string()))))
}

fn parse_rules(i: &str) -> nom::IResult<&str, HashMap<usize, Rule>> {
    let rule_p = nom::branch::alt((parse_sub_rule, parse_str_rule));
    let (i, rules) = nom::multi::separated_list1(nom::character::complete::newline, rule_p)(i)?;
    let (i, _) = nom::multi::many_m_n(2, 2, nom::character::complete::newline)(i)?;
    Ok((i, rules.into_iter().collect()))
}

const MAX_DEPTH: usize = 20;

fn build_regexp(rule_num: usize, rules: &HashMap<usize, Rule>, depth: usize) -> String {
    if depth == MAX_DEPTH {
        return String::new();
    }
    let rule = rules.get(&rule_num).unwrap();
    match rule {
        Rule::Str(s) => s.clone(),
        Rule::SubRule(sub_rules) => {
            let sr = sub_rules
                .iter()
                .map(|sr| {
                    sr.iter()
                        .map(|r| build_regexp(*r, rules, depth + 1))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("|");
            if sr.len() > 1 {
                format!("({})", sr)
            } else {
                sr
            }
        }
    }
}

fn run_1(input: &str) -> usize {
    let (messages, rules) = parse_rules(input).unwrap();
    let re = build_regexp(0, &rules, 0);
    let re = format!("^{}$", re);

    let re = Regex::new(&re).unwrap();

    messages.lines().filter(|m| re.is_match(m)).count()
}

fn run_2(input: &str) -> usize {
    let (messages, mut rules) = parse_rules(input).unwrap();
    rules.insert(8, Rule::SubRule(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::SubRule(vec![vec![42, 31], vec![42, 11, 31]]));
    let re = build_regexp(0, &rules, 0);
    let re = format!("^{}$", re);

    let re = Regex::new(&re).unwrap();

    messages.lines().filter(|m| re.is_match(m)).count()
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn aoc19_parse() {
        let (i, (rule_id, rule)) = super::parse_sub_rule("1: 2 3 | 3 2").unwrap();
        assert_eq!(rule_id, 1);
        assert!(i.is_empty());
        if let super::Rule::SubRule(rules) = rule {
            assert_eq!(rules.len(), 2);
            assert!(rules.iter().all(|r| r.len() == 2));
        } else {
            panic!();
        }

        let (messages, rules) = super::parse_rules(INPUT_1).unwrap();
        dbg! {&rules};
        assert_eq!(rules.len(), 6);
        assert_eq!(messages.lines().count(), 5);
    }

    #[test]
    fn aoc19_run_1() {
        assert_eq!(super::run_1(INPUT_1), 2);
    }

    #[test]
    fn aoc19_run_2() {
        //
    }
}
