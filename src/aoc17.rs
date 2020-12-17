use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coord4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

type Map = HashSet<Coord>;

fn parse(input: &str) -> Map {
    let mut res = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.insert(Coord {
                    x: x as i64,
                    y: y as i64,
                    z: 0,
                });
            }
        }
    }
    res
}

pub fn run() {
    let input = fs::read_to_string("day17.txt").unwrap();

    println!("day17-1: {}", run_1(&input));
    println!("day17-2: {}", run_2(&input));
}

fn add_nbrs(c: &Coord, nbrs: &mut HashMap<Coord, HashSet<Coord>>) {
    for dz in &[-1, 0, 1] {
        for dy in &[-1, 0, 1] {
            for dx in &[-1, 0, 1] {
                if (dx, dy, dz) != (&0, &0, &0) {
                    let nbr = Coord {
                        x: c.x + dx,
                        y: c.y + dy,
                        z: c.z + dz,
                    };
                    nbrs.entry((*c).clone())
                        .or_insert(HashSet::new())
                        .insert(nbr.clone());
                    nbrs.entry(nbr)
                        .or_insert(HashSet::new())
                        .insert((*c).clone());
                }
            }
        }
    }
}

fn add_nbrs4(c: &Coord4, nbrs: &mut HashMap<Coord4, HashSet<Coord4>>) {
    for dw in &[-1, 0, 1] {
        for dz in &[-1, 0, 1] {
            for dy in &[-1, 0, 1] {
                for dx in &[-1, 0, 1] {
                    if (dx, dy, dz, dw) != (&0, &0, &0, &0) {
                        let nbr = Coord4 {
                            x: c.x + dx,
                            y: c.y + dy,
                            z: c.z + dz,
                            w: c.w + dw,
                        };
                        nbrs.entry((*c).clone())
                            .or_insert(HashSet::new())
                            .insert(nbr.clone());
                        nbrs.entry(nbr)
                            .or_insert(HashSet::new())
                            .insert((*c).clone());
                    }
                }
            }
        }
    }
}

fn run_1(input: &str) -> usize {
    let mut state = parse(input);
    let mut new_state = HashSet::new();
    for _ in 0..6 {
        new_state.clear();
        let mut cache = HashMap::new();
        state.iter().for_each(|c| add_nbrs(c, &mut cache));
        for (cube, nbrs) in cache.into_iter() {
            let active_nbrs = nbrs.iter().filter(|n| state.contains(n)).count();
            if state.contains(&cube) && (active_nbrs == 2 || active_nbrs == 3) {
                new_state.insert(cube);
            } else if !state.contains(&cube) && active_nbrs == 3 {
                new_state.insert(cube);
            }
        }

        std::mem::swap(&mut state, &mut new_state);
        // dbg! {&state};
        // panic!();
    }

    state.len()
}

fn run_2(input: &str) -> usize {
    let state = parse(input);
    let mut state = state
        .iter()
        .map(|c| Coord4 {
            x: c.x,
            y: c.y,
            z: c.z,
            w: 0,
        })
        .collect::<HashSet<_>>();

    let mut new_state = HashSet::new();
    for _ in 0..6 {
        new_state.clear();
        let mut cache = HashMap::new();
        state.iter().for_each(|c| add_nbrs4(c, &mut cache));
        for (cube, nbrs) in cache.into_iter() {
            let active_nbrs = nbrs.iter().filter(|n| state.contains(n)).count();
            if state.contains(&cube) && (active_nbrs == 2 || active_nbrs == 3) {
                new_state.insert(cube);
            } else if !state.contains(&cube) && active_nbrs == 3 {
                new_state.insert(cube);
            }
        }

        std::mem::swap(&mut state, &mut new_state);
        // dbg! {&state};
        // panic!();
    }

    state.len()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = ".#.
..#
###";

    #[test]
    fn aoc17_parse() {
        let map = super::parse(INPUT);

        assert_eq!(map.len(), 5);
    }

    #[test]
    fn aoc17_run_1() {
        assert_eq!(super::run_1(INPUT), 112);
    }

    #[test]
    fn aoc17_run_2() {
        assert_eq!(super::run_2(INPUT), 848);
    }
}
