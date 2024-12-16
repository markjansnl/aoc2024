use std::collections::BTreeSet;

use nom::{
    character::complete::{anychar, newline},
    combinator::{all_consuming, map, map_res},
    error::ErrorKind,
    multi::{many1, separated_list1},
    Err,
};
use pathfinding::prelude::{astar, astar_bag};

use crate::*;

day! {
    Output = usize,
    Parsed = Map,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Position,
    end: Position,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    position: Position,
    direction: Direction,
}

impl From<Vec<Vec<Tile>>> for Map {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        Self {
            start: Self::find(&tiles, Tile::Start),
            end: Self::find(&tiles, Tile::End),
            tiles,
        }
    }
}

impl Map {
    fn find(tiles: &[Vec<Tile>], tile: Tile) -> Position {
        tiles
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find_map(|(x, &found_tile)| (found_tile == tile).then_some(Position { x, y }))
            })
            .unwrap()
    }

    fn successors(&self, node: &Node) -> Vec<(Node, usize)> {
        let mut successors = Vec::with_capacity(3);
        let mut next_position = node.position.next(node.direction);
        if self.tiles[next_position.y][next_position.x] != Tile::Wall {
            successors.push((
                Node {
                    position: next_position,
                    direction: node.direction,
                },
                1,
            ));
        }

        let mut next_direction = node.direction.rotate_cw();
        next_position = node.position.next(next_direction);
        if self.tiles[next_position.y][next_position.x] != Tile::Wall {
            successors.push((
                Node {
                    position: node.position,
                    direction: next_direction,
                },
                1000,
            ));
        }

        next_direction = node.direction.rotate_ccw();
        next_position = node.position.next(next_direction);
        if self.tiles[next_position.y][next_position.x] != Tile::Wall {
            successors.push((
                Node {
                    position: node.position,
                    direction: next_direction,
                },
                1000,
            ));
        }

        successors
    }

    fn heuristic(&self, node: &Node) -> usize {
        node.position.x.abs_diff(self.end.x) + node.position.y.abs_diff(self.end.y)
    }

    fn success(&self, node: &Node) -> bool {
        node.position == self.end
    }
}

impl Position {
    fn next(&self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

impl From<Position> for Node {
    fn from(position: Position) -> Self {
        Node {
            position,
            direction: Direction::East,
        }
    }
}

impl Day {
    fn part1(map: Parsed) -> Result<Output> {
        Ok(astar(
            &map.start.into(),
            |node| map.successors(node),
            |node| map.heuristic(node),
            |node| map.success(node),
        )
        .unwrap()
        .1)
    }

    fn part2(map: Parsed) -> Result<Output> {
        Ok(astar_bag(
            &map.start.into(),
            |node| map.successors(node),
            |node| map.heuristic(node),
            |node| map.success(node),
        )
        .unwrap()
        .0
        .flatten()
        .map(|node| node.position)
        .collect::<BTreeSet<_>>()
        .len())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::map)(input)?.1)
    }

    fn map(s: &'static str) -> IResult<Parsed> {
        map(separated_list1(newline, Self::tiles), |tiles| {
            Map::from(tiles)
        })(s)
    }

    fn tiles(s: &'static str) -> IResult<Vec<Tile>> {
        many1(map_res(anychar, |c| match c {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            _ => Err(Err::Error(("Unrecognized tile", ErrorKind::Char))),
        }))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 7036);

    test_example!("example2", Part1, 11048);

    test_example!("example1", Part2, 45);

    test_example!("example2", Part2, 64);
}
