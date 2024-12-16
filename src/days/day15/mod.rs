use std::collections::BTreeSet;

use nom::{
    character::complete::{anychar, newline},
    combinator::{all_consuming, map, map_res, opt},
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated},
    Err,
};

use crate::*;

day! {
    Output = usize,
    Parsed = Input,
}

struct Input {
    map: Map,
    directions: Vec<Direction>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
    Robot,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    robot: Position,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn next(&self, direction: Direction) -> Position {
        match direction {
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

impl Map {
    fn move_robot_part1(&mut self, direction: Direction) {
        let next = self.robot.next(direction);
        let mut next_empty = next;
        loop {
            match self.tiles[next_empty.y][next_empty.x] {
                Tile::Wall => return,
                Tile::Box => next_empty = next_empty.next(direction),
                Tile::Empty => break,
                _ => unreachable!(),
            }
        }
        self.tiles[self.robot.y][self.robot.x] = Tile::Empty;
        if next_empty != next {
            self.tiles[next.y][next.x] = Tile::Robot;
            self.tiles[next_empty.y][next_empty.x] = Tile::Box;
        } else {
            self.tiles[next_empty.y][next_empty.x] = Tile::Robot;
        }
        self.robot = next;
    }

    fn convert_to_part2(&mut self) {
        for line in self.tiles.iter_mut() {
            *line = line
                .iter()
                .flat_map(|tile| match tile {
                    Tile::Empty => [Tile::Empty, Tile::Empty],
                    Tile::Box => [Tile::BoxLeft, Tile::BoxRight],
                    Tile::Wall => [Tile::Wall, Tile::Wall],
                    Tile::Robot => [Tile::Robot, Tile::Empty],
                    _ => unreachable!(),
                })
                .collect();
        }
        self.robot.x *= 2;
    }

    fn move_robot_part2(&mut self, direction: Direction) {
        let next = self.robot.next(direction);
        match direction {
            Direction::Left | Direction::Right => {
                let mut next_empty = next;
                loop {
                    match self.tiles[next_empty.y][next_empty.x] {
                        Tile::Wall => return,
                        Tile::BoxLeft | Tile::BoxRight => {
                            next_empty = next_empty.next(direction).next(direction)
                        }
                        Tile::Empty => break,
                        _ => unreachable!(),
                    }
                }
                if direction == Direction::Left {
                    self.tiles[self.robot.y][next_empty.x..=next.x].rotate_left(1);
                } else {
                    self.tiles[self.robot.y][next.x..=next_empty.x].rotate_right(1);
                }
            }
            Direction::Up | Direction::Down => {
                let mut visited = BTreeSet::new();
                let mut changes = Vec::new();
                if self.check_move_part2(next, direction, &mut visited, &mut changes) {
                    for (position, tile) in changes {
                        self.tiles[position.y][position.x] = tile;
                    }
                } else {
                    return;
                }
            }
        }
        self.tiles[self.robot.y][self.robot.x] = Tile::Empty;
        self.robot = next;
        self.tiles[self.robot.y][self.robot.x] = Tile::Robot;
    }

    fn check_move_part2(
        &mut self,
        position: Position,
        direction: Direction,
        visited: &mut BTreeSet<Position>,
        changes: &mut Vec<(Position, Tile)>,
    ) -> bool {
        if visited.contains(&position) {
            return true;
        }

        let (current_box_left, current_box_right) = match self.tiles[position.y][position.x] {
            Tile::Empty => return true,
            Tile::BoxLeft => (position, position.next(Direction::Right)),
            Tile::BoxRight => (position.next(Direction::Left), position),
            Tile::Wall => return false,
            _ => unreachable!(),
        };

        visited.insert(current_box_left);
        visited.insert(current_box_right);

        let next_box_left = current_box_left.next(direction);
        let next_box_right = current_box_right.next(direction);

        if self.check_move_part2(next_box_left, direction, visited, changes)
            && self.check_move_part2(next_box_right, direction, visited, changes)
        {
            changes.push((current_box_left, Tile::Empty));
            changes.push((current_box_right, Tile::Empty));
            changes.push((next_box_left, Tile::BoxLeft));
            changes.push((next_box_right, Tile::BoxRight));
            true
        } else {
            false
        }
    }
}

impl Day {
    fn part1(mut input: Parsed) -> Result<Output> {
        for direction in input.directions {
            input.map.move_robot_part1(direction);
        }
        Self::gps(input.map.tiles)
    }

    fn part2(mut input: Parsed) -> Result<Output> {
        input.map.convert_to_part2();
        for direction in input.directions {
            input.map.move_robot_part2(direction);
        }
        Self::gps(input.map.tiles)
    }

    fn gps(tiles: Vec<Vec<Tile>>) -> Result<Output> {
        Ok(tiles
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.into_iter().enumerate().filter_map(move |(x, tile)| {
                    (tile == Tile::Box || tile == Tile::BoxLeft).then_some(y * 100 + x)
                })
            })
            .sum())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(map(
            separated_pair(Self::map, pair(newline, newline), Self::directions),
            |(map, directions)| Input { map, directions },
        ))(input)?
        .1)
    }

    fn map(s: &'static str) -> IResult<Map> {
        map(separated_list1(newline, Self::tiles), |tiles| {
            let robot = tiles
                .iter()
                .enumerate()
                .find_map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .find_map(|(x, &tile)| (tile == Tile::Robot).then_some(Position { x, y }))
                })
                .unwrap();
            Map { tiles, robot }
        })(s)
    }

    fn tiles(s: &'static str) -> IResult<Vec<Tile>> {
        many1(map_res(anychar, |c| match c {
            '.' => Ok(Tile::Empty),
            'O' => Ok(Tile::Box),
            '#' => Ok(Tile::Wall),
            '@' => Ok(Tile::Robot),
            _ => Err(Err::Error(("Unrecognized tile", ErrorKind::Char))),
        }))(s)
    }

    fn directions(s: &'static str) -> IResult<Vec<Direction>> {
        many1(terminated(
            map_res(anychar, |c| match c {
                '<' => Ok(Direction::Left),
                '>' => Ok(Direction::Right),
                '^' => Ok(Direction::Up),
                'v' => Ok(Direction::Down),
                _ => Err(Err::Error(("Unrecognized direction", ErrorKind::Char))),
            }),
            opt(newline),
        ))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 2028);

    test_example!("example2", Part1, 10092);

    test_example!("example2", Part2, 9021);

    test_example!("example3", Part2, 618);

    test_example!("example4", Part2, 2240);
}
