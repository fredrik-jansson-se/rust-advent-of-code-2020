use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day11.txt").unwrap();
    println!("11:1: {}", run_1(&input));
    println!("11:2: {}", run_2(&input));
}

type Coord = (isize, isize);

#[derive(Clone)]
struct Grid {
    seats: HashMap<Coord, bool>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut seats = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            height = y + 1;

            for (x, c) in line.chars().enumerate() {
                width = width.max(x + 1);
                match c {
                    'L' => {
                        seats.insert((x as isize, y as isize), false);
                    }
                    '#' => {
                        seats.insert((x as isize, y as isize), true);
                    }
                    _ => (),
                }
            }
        }
        Grid {
            seats,
            height: height as isize,
            width: width as isize,
        }
    }
    fn num_occupied(&self, (x, y): Coord) -> usize {
        let nbrs = [
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
        ];
        nbrs.iter()
            .filter(|c| *self.seats.get(c).unwrap_or(&false))
            .count()
    }
    fn num_occupied_visible(&self, (x, y): Coord) -> usize {
        let dirs = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        let mut num_visible = 0;
        for dir in &dirs {
            for i in 1..1000 {
                let pos = (x + dir.0 * i, y + dir.1 * i);
                if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width && pos.1 < self.height {
                    match self.seats.get(&pos) {
                        Some(true) => {
                            num_visible += 1;
                            break;
                        }
                        // Empty chair blocks the view, so stop looking
                        Some(false) => break,
                        None => (),
                    }
                } else {
                    break;
                }
            }
        }
        num_visible
    }
}

fn run_1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut grid_1 = grid.clone();
    let mut grid_2 = grid;
    let mut cnt = 0;
    loop {
        let (cur, next) = if cnt % 2 == 0 {
            (&grid_1, &mut grid_2)
        } else {
            (&grid_2, &mut grid_1)
        };
        cnt += 1;

        for (c, v) in cur.seats.iter() {
            match (v, cur.num_occupied(*c)) {
                (false, 0) => {
                    next.seats.insert(*c, true);
                }
                (true, o) if o > 3 => {
                    next.seats.insert(*c, false);
                }
                _ => {
                    next.seats.insert(*c, *v);
                }
            }
        }
        if cur.seats == next.seats {
            break;
        }
    }

    grid_1.seats.iter().filter(|(_, taken)| **taken).count()
}

fn run_2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut grid_1 = grid.clone();
    let mut grid_2 = grid;
    let mut cnt = 0;
    loop {
        let (cur, next) = if cnt % 2 == 0 {
            (&grid_1, &mut grid_2)
        } else {
            (&grid_2, &mut grid_1)
        };
        cnt += 1;

        for (c, v) in cur.seats.iter() {
            match (v, cur.num_occupied_visible(*c)) {
                (false, 0) => {
                    next.seats.insert(*c, true);
                }
                (true, o) if o > 4 => {
                    next.seats.insert(*c, false);
                }
                _ => {
                    next.seats.insert(*c, *v);
                }
            }
        }
        if cur.seats == next.seats {
            break;
        }
    }

    grid_1.seats.iter().filter(|(_, taken)| **taken).count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn aoc11_run_1() {
        assert_eq!(super::run_1(INPUT), 37)
    }

    #[test]
    fn aoc11_occupied_visible() {
        let grid = super::Grid::new(
            ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
        );
        let visible = grid.seats.iter().find(|(_, v)| **v == false).unwrap();
        assert_eq!(grid.num_occupied_visible(*visible.0), 8);

        let grid = super::Grid::new(
            ".............
.L.L.#.#.#.#.
.............",
        );
        assert_eq!(grid.num_occupied_visible((1, 1)), 0);
        assert_eq!(grid.num_occupied_visible((3, 1)), 1);

        let grid = super::Grid::new(
            ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
        );
        assert_eq!(grid.num_occupied_visible((3, 3)), 0);
    }

    #[test]
    fn aoc11_run_2() {
        assert_eq!(super::run_2(INPUT), 26)
    }
}
