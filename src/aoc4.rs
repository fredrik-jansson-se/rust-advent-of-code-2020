use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::space1,
    character::complete::{char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day4.txt").unwrap();
    println!("4:1 {}", run_1(&input));
    println!("4:2 {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let pp = parse_passports(input).unwrap().1;
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    pp.iter()
        .filter(|passport| {
            let keys: Vec<&str> = passport.iter().map(|(k, _)| *k).collect();
            required.iter().all(|req| keys.contains(req))
        })
        .count()
}

fn val_range(v: &str, len: usize, low: usize, high: usize) -> bool {
    let re = format!("^[0-9]{{{}}}$", len);
    let re = regex::Regex::new(&re).unwrap();
    if !re.is_match(v) {
        return false;
    }
    v.parse::<usize>()
        .map(|v| v >= low && v <= high)
        .unwrap_or(false)
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => val_range(value, 4, 1920, 2002),
        "iyr" => val_range(value, 4, 2010, 2020),
        "eyr" => val_range(value, 4, 2020, 2030),
        "hgt" => {
            let cm = regex::Regex::new("^([0-9]+)cm$").unwrap();
            let inch = regex::Regex::new("^([0-9]+)in$").unwrap();
            match (
                cm.captures_iter(value).next(),
                inch.captures_iter(value).next(),
            ) {
                (Some(c), _) => val_range(c.get(1).unwrap().as_str(), 3, 150, 193),
                (_, Some(c)) => val_range(c.get(1).unwrap().as_str(), 2, 59, 76),
                _ => false,
            }
        }
        "hcl" => {
            let re = regex::Regex::new("^#[0-9a-f]{6}$").unwrap();
            re.is_match(value)
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => {
            let re = regex::Regex::new("^[0-9]{9}$").unwrap();
            re.is_match(value)
        }
        "cid" => true,
        _ => unreachable!(),
    }
}

fn run_2(input: &str) -> usize {
    let pp = parse_passports(input).unwrap().1;
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valids = pp
        .iter()
        .filter_map(|passport| {
            let passport = passport
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<HashMap<String, String>>();
            if required.iter().all(|r| passport.contains_key(*r)) {
                Some(passport)
            } else {
                None
            }
        })
        .filter(|passport| passport.iter().all(|(k, v)| validate_field(k, v)));
    valids.count()
}

fn parse_val(i: &str) -> IResult<&str, &str> {
    take_till1(|c| match c {
        ':' | ' ' | '\n' => true,
        _ => false,
    })(i)
}

fn parse_kv(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(parse_val, tag(":"), parse_val)(i)
}

fn parse_passport(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(alt((newline, char(' '))), parse_kv)(i)
}

fn parse_passports(i: &str) -> IResult<&str, Vec<Vec<(&str, &str)>>> {
    separated_list1(tag("\n\n"), parse_passport)(i)
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn aoc4_parse() {
        assert_eq!(super::parse_kv("ecl:gry").unwrap().1, ("ecl", "gry"));
        assert_eq!(
            super::parse_kv("hcl:#fffffd").unwrap().1,
            ("hcl", "#fffffd")
        );

        let (_, passport) = super::parse_passport(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        )
        .unwrap();
        assert_eq!(passport.len(), 8);

        let input = "a:b
c:d

e:f";
        let (i, passports) = super::parse_passports(&input).unwrap();
        assert_eq!(i, "");
        assert_eq!(passports.len(), 2);
        let (_, passports) = super::parse_passports(INPUT).unwrap();
        assert_eq!(passports.len(), 4);
    }
    #[test]
    fn aoc4_run_1() {
        assert_eq!(super::run_1(INPUT), 2);
    }
    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    #[test]
    fn aoc2_test_valid() {
        assert!(super::validate_field("byr", "1920"));
        assert!(!super::validate_field("byr", "1919"));
        assert!(super::validate_field("byr", "2002"));
        assert!(!super::validate_field("byr", "2003"));

        assert!(super::validate_field("hgt", "60in"));
        assert!(super::validate_field("hgt", "190cm"));
        assert!(!super::validate_field("hgt", "60inch"));
        assert!(!super::validate_field("hgt", "190in"));
        assert!(!super::validate_field("hgt", "190"));

        assert!(super::validate_field("hcl", "#123abc"));
        assert!(!super::validate_field("hcl", "#123abcd"));
        assert!(!super::validate_field("hcl", "#123abz"));
        assert!(!super::validate_field("hcl", "123abc"));

        assert!(super::validate_field("ecl", "brn"));
        assert!(!super::validate_field("ecl", "wat"));

        assert!(super::validate_field("pid", "000000001"));
        assert!(!super::validate_field("pid", "0123456789"));
    }

    #[test]
    fn aoc4_run_2() {
        assert_eq!(super::run_2(INVALID), 0);
        assert_eq!(super::run_2(VALID), 4);
    }
}
