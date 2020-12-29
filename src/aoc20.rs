use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

pub fn run() {
    let input = fs::read_to_string("day20.txt").unwrap();
    println!("20:1: {}", run_1(&input));
    println!("20:2: {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let (_, all_directions) = parse(input).unwrap();

    let mut tiles = HashSet::new();

    for directions in all_directions.iter() {
        let mut pos = Pos { row: 0, col: 0 };
        for d in directions.iter() {
            let even_row = pos.row % 2 == 0;
            match d {
                Direction::NW if even_row => {
                    pos.row -= 1;
                    pos.col -= 1;
                }
                Direction::NW => {
                    pos.row -= 1;
                }
                Direction::NE if even_row => {
                    pos.row -= 1;
                }
                Direction::NE => {
                    pos.row -= 1;
                    pos.col += 1;
                }
                Direction::SW if even_row => {
                    pos.row += 1;
                    pos.col -= 1;
                }
                Direction::SW => {
                    pos.row += 1;
                }
                Direction::SE if even_row => {
                    pos.row += 1;
                }
                Direction::SE => {
                    pos.row += 1;
                    pos.col += 1;
                }
                Direction::W => {
                    pos.col -= 1;
                }
                Direction::E => {
                    pos.col += 1;
                }
            }
        }
        if tiles.contains(&pos) {
            tiles.remove(&pos);
        } else {
            tiles.insert(pos);
        }
    }
    tiles.len()
}

fn run_2(_input: &str) -> i64 {
    unreachable!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    NW,
    NE,
    W,
    E,
    SW,
    SE,
}

fn parse(i: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    let nw = map(tag("nw"), |_| Direction::NW);
    let ne = map(tag("ne"), |_| Direction::NE);
    let sw = map(tag("sw"), |_| Direction::SW);
    let se = map(tag("se"), |_| Direction::SE);
    let w = map(tag("w"), |_| Direction::W);
    let e = map(tag("e"), |_| Direction::E);

    let line = many1(alt((nw, ne, sw, se, w, e)));

    separated_list1(newline, line)(i)
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc20_run_1() {
        let ans = super::run_1(
            "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
        );
        assert_eq!(ans, 10);
    }
}
