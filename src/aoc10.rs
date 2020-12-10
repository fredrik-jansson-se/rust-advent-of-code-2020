use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day10.txt").unwrap();
    println!("10:1 {}", run_1(&input));
    println!("10:2 {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let (_, mut input) = parse(input).unwrap();
    input.push(0);
    input.sort();
    let built_in = input.iter().max().unwrap() + 3;
    input.push(built_in);

    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for (i, v) in input.iter().enumerate().skip(1) {
        match v - input[i - 1] {
            1 => {
                diff_1 += 1;
            }
            3 => {
                diff_3 += 1;
            }
            _ => (),
        }
    }

    diff_1 * diff_3
}

fn combinations(v: usize, vals: &[usize], cache: &mut HashMap<usize, u128>) -> u128 {
    if vals.is_empty() {
        return 1;
    }

    if let Some(combs) = cache.get(&v) {
        return *combs;
    }

    let mut combs = 0;

    for (i, v1) in vals.iter().enumerate() {
        if (v - v1) > 3 {
            break;
        }
        combs += combinations(*v1, &vals[(i + 1)..], cache);
    }

    cache.insert(v, combs);

    combs
}

fn run_2(input: &str) -> u128 {
    let (_, mut input) = parse(input).unwrap();
    input.push(0);
    input.sort();
    let built_in = input.iter().max().unwrap() + 3;
    let input = input.into_iter().rev().collect::<Vec<_>>();
    let mut cache = HashMap::new();
    combinations(built_in, &input, &mut cache)
}

fn parse(i: &str) -> nom::IResult<&str, Vec<usize>> {
    nom::multi::separated_list1(
        nom::character::complete::newline,
        crate::helper::uval::<usize>,
    )(i)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn aoc10_combinations() {
        use std::collections::HashMap;
        let mut cache = HashMap::new();
        assert_eq!(super::combinations(3, &[0], &mut cache), 1);
        let mut cache = HashMap::new();
        assert_eq!(super::combinations(5, &[0], &mut cache), 0);
        let mut cache = HashMap::new();
        assert_eq!(super::combinations(5, &[3, 0], &mut cache), 1);
        let mut cache = HashMap::new();
        assert_eq!(super::combinations(5, &[3, 2, 0], &mut cache), 3);
    }

    #[test]
    fn aoc10_run_1() {
        assert_eq!(super::run_1(INPUT_1), 7 * 5);
        assert_eq!(super::run_1(INPUT_2), 220);
    }

    #[test]
    fn aoc10_run_2() {
        assert_eq!(super::run_2(INPUT_1), 8);
        assert_eq!(super::run_2(INPUT_2), 19208);
    }
}
