use crate::helper::*;
use std::fs;

use nom::{
    branch::alt,
    character::complete::{char, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Cmd {
    East(isize),
    West(isize),
    North(isize),
    South(isize),
    Forward(isize),
    Left(isize),
    Rigth(isize),
}

macro_rules! cmd {
    ($fun:ident, $c:literal, $cmd:expr) => {
        fn $fun(i: &str) -> IResult<&str, Cmd> {
            let (i, _) = char($c)(i)?;
            let (i, m) = ival::<isize>(i)?;
            Ok((i, $cmd(m)))
        }
    };
}

cmd!(cmd_e, 'E', Cmd::East);
cmd!(cmd_w, 'W', Cmd::West);
cmd!(cmd_n, 'N', Cmd::North);
cmd!(cmd_s, 'S', Cmd::South);
cmd!(cmd_l, 'L', Cmd::Left);
cmd!(cmd_r, 'R', Cmd::Rigth);
cmd!(cmd_f, 'F', Cmd::Forward);

fn parse(i: &str) -> IResult<&str, Vec<Cmd>> {
    separated_list1(
        newline,
        alt((cmd_e, cmd_w, cmd_n, cmd_s, cmd_l, cmd_r, cmd_f)),
    )(i)
}

pub fn run() {
    let input = fs::read_to_string("day12.txt").unwrap();
    println!("12:1 {}", run_1(&input));
    println!("12:2 {}", run_2(&input));
}

type Coord = (isize, isize);

#[derive(Debug)]
struct State {
    x: isize,
    y: isize,
    dir: (isize, isize),
}

fn rotate_ccw((mut x, mut y): Coord, a: isize) -> Coord {
    let mut a = if a < 0 { 360 + a } else { a };
    while a > 0 {
        let (nx, ny) = (-y, x);
        x = nx;
        y = ny;
        a -= 90;
    }
    (x, y)
}

impl State {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: (1, 0),
        }
    }

    fn take_command(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::North(v) => self.y += v,
            Cmd::South(v) => self.y -= v,
            Cmd::West(v) => self.x -= v,
            Cmd::East(v) => self.x += v,
            Cmd::Forward(v) => {
                self.x += v * self.dir.0;
                self.y += v * self.dir.1;
            }
            Cmd::Left(v) => self.dir = rotate_ccw(self.dir, *v),
            Cmd::Rigth(v) => self.dir = rotate_ccw(self.dir, -v),
        }
    }
}

fn run_1(input: &str) -> isize {
    let (_, cmds) = parse(input).unwrap();
    let mut state = State::new();
    for cmd in cmds {
        state.take_command(&cmd);
    }

    state.x.abs() + state.y.abs()
}

fn run_2(input: &str) -> isize {
    let (_, cmds) = parse(input).unwrap();
    let mut ship_pos = (0, 0);
    let mut waypt = (10, 1);
    for cmd in cmds {
        match cmd {
            Cmd::North(v) => waypt.1 += v,
            Cmd::South(v) => waypt.1 -= v,
            Cmd::West(v) => waypt.0 -= v,
            Cmd::East(v) => waypt.0 += v,
            Cmd::Forward(v) => {
                let dir = (waypt.0, waypt.1);
                ship_pos.0 += v * dir.0;
                ship_pos.1 += v * dir.1;
            }
            Cmd::Left(v) => {
                waypt = rotate_ccw(waypt, v);
            }
            Cmd::Rigth(v) => {
                waypt = rotate_ccw(waypt, -v);
            }
        }
    }
    ship_pos.0.abs() + ship_pos.1.abs()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn aoc12_parse() {
        let (_, cmds) = super::parse(INPUT).unwrap();
        assert_ne!(cmds.len(), 4);
    }

    #[test]
    fn aoc12_run_1() {
        assert_eq!(super::run_1(INPUT), 25);
    }

    #[test]
    fn aoc12_run_2() {
        assert_eq!(super::run_2(INPUT), 286);
    }
}
