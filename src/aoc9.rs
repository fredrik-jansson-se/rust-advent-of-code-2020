use std::fs;

pub fn run() {
    let input = fs::read_to_string("day9.txt").unwrap();
    println!("day9-1: {}", run_1(&input, 100));
    println!("day9-2: {}", run_2(&input, 100));
}

fn is_valid(v: usize, preamble: &[usize]) -> bool {
    for (i, v1) in preamble.iter().enumerate() {
        if v > *v1 {
            // dbg! {(v, v1, v-v1)};
            if preamble[(i + 1)..].contains(&(v - v1)) {
                return true;
            }
        }
    }
    return false;
}

fn run_1(input: &str, preamble_len: usize) -> usize {
    let (_, xmas) = parse_xmas(input).unwrap();
    let mut start = 0;
    loop {
        let end = start + preamble_len;
        let preamble = &xmas[start..end];
        // dbg! {(xmas[end], preamble)};
        if !is_valid(xmas[end], preamble) {
            return xmas[end];
        }

        start += 1;
    }
}

fn run_2(input: &str, preamble_len: usize) -> usize {
    let (_, xmas) = parse_xmas(input).unwrap();
    let invalid = run_1(input, preamble_len);

    for (i, v1) in xmas.iter().enumerate() {
        let mut min = v1;
        let mut max = v1;
        let mut sum = *v1;
        for v2 in xmas[(i + 1)..].iter() {
            min = min.min(v2);
            max = max.max(v2);
            sum += *v2;
            if sum > invalid {
                break;
            }
            if sum == invalid {
                return min + max;
            }
        }
    }
    unreachable!()
}

fn parse_xmas(i: &str) -> nom::IResult<&str, Vec<usize>> {
    nom::multi::separated_list1(
        nom::character::complete::newline,
        crate::helper::uval::<usize>,
    )(i)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    #[test]
    fn aoc9_is_valid() {
        assert!(super::is_valid(6, &[1, 2, 3, 4, 5]));
        assert!(!super::is_valid(10, &[1, 2, 3, 4, 5]));
    }

    #[test]
    fn aoc9_run_1() {
        assert_eq!(super::run_1(INPUT, 5), 127);
    }

    #[test]
    fn aoc9_run_2() {
        assert_eq!(super::run_2(INPUT, 5), 62);
    }
}
