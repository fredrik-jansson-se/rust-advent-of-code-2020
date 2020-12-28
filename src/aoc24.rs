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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

pub fn run() {
    let input = fs::read_to_string("day24.txt").unwrap();
    println!("24:1: {}", run_1(&input));
    println!("24:2: {}", run_2(&input));
}

fn follow_directions(input: &str) -> HashSet<Pos> {
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
    tiles
}

fn run_1(input: &str) -> usize {
    let tiles = follow_directions(input);
    tiles.len()
}

fn neighbors(pos: &Pos) -> Vec<Pos> {
    if pos.row % 2 == 0 {
        // Even row
        vec![
            Pos {
                col: pos.col - 1,
                row: pos.row - 1,
            },
            Pos {
                col: pos.col,
                row: pos.row - 1,
            },
            Pos {
                col: pos.col - 1,
                row: pos.row + 1,
            },
            Pos {
                col: pos.col,
                row: pos.row + 1,
            },
            Pos {
                col: pos.col - 1,
                row: pos.row,
            },
            Pos {
                col: pos.col + 1,
                row: pos.row,
            },
        ]
    } else {
        vec![
            Pos {
                col: pos.col,
                row: pos.row - 1,
            },
            Pos {
                col: pos.col + 1,
                row: pos.row - 1,
            },
            Pos {
                col: pos.col,
                row: pos.row + 1,
            },
            Pos {
                col: pos.col + 1,
                row: pos.row + 1,
            },
            Pos {
                col: pos.col - 1,
                row: pos.row,
            },
            Pos {
                col: pos.col + 1,
                row: pos.row,
            },
        ]
    }
}

fn run_2(input: &str) -> usize {
    let mut tiles = follow_directions(input);

    for i in 1..=100 {
        let mut new_tiles = HashSet::new();

        for pos in tiles.iter() {
            let nbrs = neighbors(pos);
            let (black_nbrs, white_nbrs): (Vec<&Pos>, Vec<&Pos>) =
                nbrs.iter().partition(|p| tiles.contains(p));

            // We know that pos is black
            // any black tile with zero or more than 2 black tiles
            // immediately adjacent to it is flipped to white.
            // i.e. if it has one or two neighbors, it survives
            if black_nbrs.len() == 1 || black_nbrs.len() == 2 {
                new_tiles.insert((*pos).clone());
            }

            for white_nbr in white_nbrs.into_iter() {
                let num_black_nbrs = neighbors(white_nbr)
                    .iter()
                    .filter(|p| tiles.contains(p))
                    .count();
                if num_black_nbrs == 2 {
                    new_tiles.insert(white_nbr.clone());
                }
            }
        }

        tiles = new_tiles;
    }
    tiles.len()
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
    fn aoc24_run_1() {
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

    #[test]
    fn aoc24_run_2() {
        let ans = super::run_2(
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
        assert_eq!(ans, 2208);
    }
}
