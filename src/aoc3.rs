use std::fs;

type Map = Vec<Vec<char>>;

pub fn run() {
    let input = fs::read_to_string("day3.txt").unwrap();
    let map = parse(&input);

    println!("3:1 - {}", run_1(&map));
    println!("3:2 - {}", run_2(&map));
}

fn count_trees(map: &Map, slope: (usize, usize)) -> usize {
    let mut pos = (0, 0);
    let mut tree_count = 0;
    let width = map[0].len();
    while pos.1 < map.len() {
        if map[pos.1][pos.0 % width] == '#' {
            tree_count += 1;
        }
        pos.0 += slope.0;
        pos.1 += slope.1;
    }
    tree_count
}

fn run_1(map: &Map) -> usize {
    count_trees(map, (3, 1))
}

fn run_2(map: &Map) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .fold(1, |acc, slope| acc * count_trees(map, *slope))
}

fn parse(input: &str) -> Map {
    let mut res = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let row = line.chars().collect();
        res.push(row);
    }

    res
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn aoc3_parse() {
        let map = super::parse(INPUT);
        assert_eq!(11, map.len());
        assert_eq!(11, map[2].len());
    }

    #[test]
    fn aoc3_run_1() {
        let map = super::parse(INPUT);
        assert_eq!(super::run_1(&map), 7);
    }

    #[test]
    fn aoc3_run_2() {
        let map = super::parse(INPUT);
        assert_eq!(super::run_2(&map), 336);
    }
}
