mod util;
use pathfinding::prelude::astar;
use util::*;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u8},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};

use crate::*;

day! {
    Output = String,
    Parsed = Vec<Position>,
}

struct Config {
    size: usize,
    coordinates: usize,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    #[default]
    Empty,
    Corrupted,
}

struct MemorySpace {
    bytes: Grid<71, 71, Tile>,
}

impl MemorySpace {
    fn with_size(size: usize) -> Self {
        Self {
            bytes: Grid::with_size(size, size),
        }
    }
}

impl Day {
    fn part1(coordinates: Parsed) -> Result<Output> {
        let config = Self::get_config(&coordinates);
        let mut memory_space = MemorySpace::with_size(config.size);
        for position in coordinates.iter().copied().take(config.coordinates) {
            memory_space.bytes[position] = Tile::Corrupted;
        }

        let start = memory_space.bytes.position(Position::default());
        let end = memory_space.bytes.position(Position {
            x: config.size - 1,
            y: config.size - 1,
        });
        let (_, length) = astar(
            &start,
            |pos| {
                [pos.left(), pos.right(), pos.up(), pos.down()]
                    .into_iter()
                    .filter_map(|next| {
                        if let Some(next_pos) = next {
                            if memory_space.bytes[next_pos] != Tile::Corrupted {
                                return Some((next_pos, 1));
                            }
                        }
                        None
                    })
            },
            |pos| pos.manhatten_distance(end),
            |pos| *pos == end,
        )
        .unwrap();

        Ok(length.to_string())
    }

    fn part2(coordinates: Parsed) -> Result<Output> {
        let config = Self::get_config(&coordinates);
        let mut memory_space = MemorySpace::with_size(config.size);
        let mut coordinates_iter = coordinates.iter().copied();
        for _ in 0..config.coordinates {
            let position = coordinates_iter.next().unwrap();
            memory_space.bytes[position] = Tile::Corrupted;
        }

        let end = Position {
            x: config.size - 1,
            y: config.size - 1,
        };

        let mut last_path = Vec::new();

        while let Some(position) = coordinates_iter.next() {
            memory_space.bytes[position] = Tile::Corrupted;

            if !last_path.is_empty() && last_path.iter().copied().all(|pos| pos != position) {
                continue;
            }

            let start_pos = memory_space.bytes.position(Position::default());
            let end_pos = memory_space.bytes.position(end);

            if let Some((path, _)) = astar(
                &start_pos,
                |pos| {
                    [pos.left(), pos.right(), pos.up(), pos.down()]
                        .into_iter()
                        .filter_map(|next| {
                            if let Some(next_pos) = next {
                                if memory_space.bytes[next_pos] != Tile::Corrupted {
                                    return Some((next_pos, 1));
                                }
                            }
                            None
                        })
                },
                |pos| pos.manhatten_distance(end_pos),
                |pos| *pos == end_pos,
            ) {
                last_path = path.into_iter().map(|pos| pos.position()).collect();
            } else {
                return Ok(format!("{},{}", position.x, position.y));
            }
        }
        unreachable!()
    }

    fn get_config(coordinates: &Parsed) -> Config {
        if coordinates.len() == 25 {
            Config {
                size: 7,
                coordinates: 12,
            }
        } else {
            Config {
                size: 71,
                coordinates: 1024,
            }
        }
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::coordinates)(input)?.1)
    }

    fn coordinates(s: &'static str) -> IResult<Parsed> {
        separated_list1(newline, Self::coordinate)(s)
    }

    fn coordinate(s: &'static str) -> IResult<Position> {
        map(separated_pair(u8, tag(","), u8), |(x, y)| Position {
            x: x as usize,
            y: y as usize,
        })(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 22);

    test_example!("example1", Part2, "6,1");
}
