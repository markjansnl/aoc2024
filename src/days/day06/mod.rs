use std::collections::HashSet;

use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use crate::*;

day! {
    Output = usize,
    Parsed = Vec<Vec<Tile>>,
    bench_sample_size: 50,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Obstruction,
    Guard,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    #[inline]
    fn rotate_clockwise(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Location {
    #[inline]
    fn next(&self, direction: Direction) -> Location {
        let Location { x, y } = *self;
        match direction {
            Direction::North => Location { x, y: y - 1 },
            Direction::East => Location { x: x + 1, y },
            Direction::South => Location { x, y: y + 1 },
            Direction::West => Location { x: x - 1, y },
        }
    }
}

impl Day {
    #[inline]
    fn part1(lines: Parsed) -> Result<Output> {
        let mut location = Self::find_guard(&lines)?;
        let mut direction = Direction::North;
        let mut visited = HashSet::from([location]);

        loop {
            let next = location.next(direction);
            match Self::get(&lines, next) {
                None => break,
                Some(Tile::Obstruction) => direction.rotate_clockwise(),
                _ => {
                    visited.insert(next);
                    location = next
                }
            }
        }
        Ok(visited.len())
    }

    #[inline]
    fn part2(lines: Parsed) -> Result<Output> {
        let mut location = Self::find_guard(&lines)?;
        let mut direction = Direction::North;
        let mut turns = HashSet::new();
        let mut visited = HashSet::from([location]);
        let mut extra_obstructions = HashSet::new();

        loop {
            let next = location.next(direction);
            match Self::get(&lines, next) {
                None => break,
                Some(Tile::Obstruction) => {
                    turns.insert((location, direction));
                    direction.rotate_clockwise();
                }
                _ => {
                    if !visited.contains(&next)
                        && Self::can_place_obstruction(&lines, location, direction, &turns)
                    {
                        extra_obstructions.insert(next);
                    }
                    visited.insert(next);
                    location = next
                }
            }
        }
        Ok(extra_obstructions.len())
    }

    #[inline]
    fn find_guard(lines: &Parsed) -> Result<Location> {
        for (y, tiles) in lines.iter().enumerate() {
            for (x, tile) in tiles.iter().enumerate() {
                if *tile == Tile::Guard {
                    return Ok(Location {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }
        bail!("Guard not found");
    }

    #[inline]
    fn get(lines: &Parsed, Location { x, y }: Location) -> Option<Tile> {
        if y < 0 || y >= lines.len() as isize || x < 0 || x >= lines[0].len() as isize {
            None
        } else {
            Some(lines[y as usize][x as usize])
        }
    }

    #[inline]
    fn can_place_obstruction(
        lines: &Parsed,
        location: Location,
        direction: Direction,
        turns: &HashSet<(Location, Direction)>,
    ) -> bool {
        let mut location = location;
        let mut direction = direction;
        let mut turns = turns.clone();
        let obstruction = location.next(direction);

        loop {
            let next = location.next(direction);
            if turns.contains(&(next, direction)) {
                return true;
            }
            match Self::get(lines, next) {
                None => return false,
                Some(Tile::Obstruction) => {
                    turns.insert((location, direction));
                    direction.rotate_clockwise()
                }
                _ => {
                    if next == obstruction {
                        turns.insert((location, direction));
                        direction.rotate_clockwise();
                    } else {
                        location = next
                    }
                }
            }
        }
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::lines)(input)?.1)
    }

    #[inline]
    fn lines(s: &'static str) -> IResult<Parsed> {
        separated_list1(newline, Self::tiles)(s)
    }

    #[inline]
    fn tiles(s: &'static str) -> IResult<Vec<Tile>> {
        many1(Self::tile)(s)
    }

    #[inline]
    fn tile(s: &'static str) -> IResult<Tile> {
        alt((
            map(tag("#"), |_| Tile::Obstruction),
            map(tag("."), |_| Tile::Empty),
            map(tag("^"), |_| Tile::Guard),
        ))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 41);

    test_example!("example1", Part2, 6);
}
