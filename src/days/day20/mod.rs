use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::*;

day! {
    Output = usize,
    Parsed = Input,
    bench_sample_size: 10,
}

#[derive(Default)]
struct Input {
    start: Position,
    end: Position,
    track: BTreeSet<Position>,
    is_example: bool,
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
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
}

impl Day {
    fn part1(input: Parsed) -> Result<Output> {
        Self::count_cheats(&input, 2, if input.is_example { 1 } else { 100 })
    }

    fn part2(input: Parsed) -> Result<Output> {
        Self::count_cheats(&input, 20, if input.is_example { 50 } else { 100 })
    }

    fn count_cheats(input: &Parsed, cheat_length: isize, min_picoseconds: isize) -> Result<Output> {
        let mut position = input.end;
        let mut steps = 0;
        let mut visited = BTreeMap::new();
        let mut cheats = HashSet::new();

        visited.insert(position, 0);

        while position != input.start {
            for next in [
                position.left(),
                position.right(),
                position.up(),
                position.down(),
            ] {
                if !input.track.contains(&next) || visited.contains_key(&next) {
                    continue;
                }

                steps += 1;
                position = next;
                visited.insert(position, steps);

                for next_y in position.y - cheat_length..=position.y + cheat_length {
                    for next_x in position.x - cheat_length..=position.x + cheat_length {
                        let next = Position {
                            x: next_x,
                            y: next_y,
                        };
                        let manhatten_distance = position.manhatten_distance(next);
                        if manhatten_distance <= cheat_length {
                            if let Some(steps_next) = visited.get(&next) {
                                if steps - manhatten_distance - steps_next >= min_picoseconds {
                                    cheats.insert((position, next));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(cheats.len())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let mut parsed = Input::default();
        parsed.is_example = input.lines().count() == 15;
        for (y, line) in input.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'.' => {
                        parsed.track.insert(Position {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                    b'S' => {
                        parsed.start = Position {
                            x: x as isize,
                            y: y as isize,
                        };
                        parsed.track.insert(parsed.start);
                    }
                    b'E' => {
                        parsed.end = Position {
                            x: x as isize,
                            y: y as isize,
                        };
                        parsed.track.insert(parsed.end);
                    }
                    b'#' => {}
                    _ => panic!("{b} is not a valid input character."),
                }
            }
        }
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 44);

    test_example!("example1", Part2, 285);
}
