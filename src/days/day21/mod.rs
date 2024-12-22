use std::collections::{BTreeMap, BTreeSet};

use memoize::memoize;
use pathfinding::prelude::astar_bag;

use crate::*;

day! {
    Output = usize,
    Parsed = Vec<Vec<char>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Keypad {
    keys: BTreeMap<char, Position>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum KeypadType {
    Numeric,
    Directional,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Keypad {
    fn new_numeric() -> Self {
        Self::new([
            [Some('7'), Some('8'), Some('9')],
            [Some('4'), Some('5'), Some('6')],
            [Some('1'), Some('2'), Some('3')],
            [None, Some('0'), Some('A')],
        ])
    }

    fn new_directional() -> Self {
        Self::new([
            [None, Some('^'), Some('A')],
            [Some('<'), Some('v'), Some('>')],
        ])
    }

    fn new(keys: impl IntoIterator<Item = [Option<char>; 3]>) -> Self {
        Self {
            keys: keys
                .into_iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter(|(_, c)| c.is_some())
                        .map(move |(x, c)| {
                            (
                                c.unwrap(),
                                Position {
                                    x: x as isize,
                                    y: y as isize,
                                },
                            )
                        })
                })
                .collect(),
        }
    }
}

impl From<KeypadType> for Keypad {
    fn from(keypad_type: KeypadType) -> Self {
        match keypad_type {
            KeypadType::Numeric => Keypad::new_numeric(),
            KeypadType::Directional => Keypad::new_directional(),
        }
    }
}

impl Position {
    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn manhatten_distance(&self, rhs: Position) -> isize {
        (self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)) as isize
    }

    fn key(&self, rhs: Position) -> char {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;

        match (dx, dy) {
            (-1, 0) => '<',
            (1, 0) => '>',
            (0, -1) => '^',
            (0, 1) => 'v',
            _ => unreachable!(),
        }
    }
}

impl Day {
    fn part1(codes: Parsed) -> Result<Output> {
        Self::complexities(codes, 4)
    }

    fn part2(codes: Parsed) -> Result<Output> {
        Self::complexities(codes, 27)
    }

    fn complexities(codes: Parsed, depth: usize) -> Result<Output> {
        Ok(codes
            .into_iter()
            .map(|code| {
                let code_num = code[0].to_digit(10).unwrap() as usize * 100
                    + code[1].to_digit(10).unwrap() as usize * 10
                    + code[2].to_digit(10).unwrap() as usize;
                let steps = code_steps(KeypadType::Numeric, 'A', code.clone(), depth);
                code_num * steps
            })
            .sum())
    }
}

#[memoize]
fn code_steps(keypad_type: KeypadType, prev: char, code: Vec<char>, depth: usize) -> usize {
    if depth == 0 {
        1
    } else if code.is_empty() {
        0
    } else {
        let mut code = code.clone();
        let next = code.remove(0);

        let inner_steps = routes(keypad_type, prev, next)
            .into_iter()
            .map(|inner_code| {
                code_steps(KeypadType::Directional, 'A', inner_code.clone(), depth - 1)
            })
            .min()
            .unwrap();

        inner_steps + code_steps(keypad_type, next, code, depth)
    }
}

#[memoize]
fn routes(keypad_type: KeypadType, start: char, end: char) -> Vec<Vec<char>> {
    let keypad = Keypad::from(keypad_type);
    let valid_positions = keypad.keys.values().collect::<BTreeSet<_>>();

    astar_bag(
        &keypad.keys[&start],
        |pos| {
            [pos.left(), pos.right(), pos.up(), pos.down()]
                .into_iter()
                .filter(|next| valid_positions.contains(&next))
                .map(|next| (next, 1))
        },
        |pos| keypad.keys[&end].manhatten_distance(*pos),
        |pos| *pos == keypad.keys[&end],
    )
    .unwrap()
    .0
    .map(|positions| {
        let mut route = positions
            .windows(2)
            .map(|w| w[0].key(w[1]))
            .collect::<Vec<_>>();
        route.push('A');
        route
    })
    .collect()
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(input.lines().map(|line| line.chars().collect()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 126384);
}
