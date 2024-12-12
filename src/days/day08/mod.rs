use std::collections::{HashMap, HashSet};

use crate::*;

day! {
    Output = usize,
    Parsed = Map,
}

struct Map {
    width: isize,
    height: isize,
    antennas: HashMap<Antenna, Vec<Location>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna(char);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Map {
    fn contains(&self, location: Location) -> bool {
        location.x >= 0 && location.x < self.width && location.y >= 0 && location.y < self.height
    }
}

impl Day {
    fn part1(map: Parsed) -> Result<Output> {
        let mut antinodes = HashSet::new();
        for locations in map.antennas.values() {
            for (idx, a) in locations.iter().enumerate() {
                for b in &locations[idx + 1..] {
                    let antinode1 = Self::next_antinode(*a, *b);
                    if map.contains(antinode1) {
                        antinodes.insert(antinode1);
                    }

                    let antinode2 = Self::next_antinode(*b, *a);
                    if map.contains(antinode2) {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }

        Ok(antinodes.len())
    }

    fn part2(map: Parsed) -> Result<Output> {
        let mut antinodes = HashSet::new();
        for locations in map.antennas.values() {
            for (idx, a) in locations.iter().enumerate() {
                for b in &locations[idx + 1..] {
                    antinodes.insert(*a);
                    antinodes.insert(*b);

                    let mut k = *a;
                    let mut l = *b;
                    let mut m = Self::next_antinode(k, l);
                    while map.contains(m) {
                        antinodes.insert(m);
                        k = l;
                        l = m;
                        m = Self::next_antinode(k, l);
                    }

                    k = *b;
                    l = *a;
                    let mut m = Self::next_antinode(k, l);
                    while map.contains(m) {
                        antinodes.insert(m);
                        k = l;
                        l = m;
                        m = Self::next_antinode(k, l);
                    }
                }
            }
        }

        Ok(antinodes.len())
    }

    fn next_antinode(a: Location, b: Location) -> Location {
        Location {
            x: 2 * b.x - a.x,
            y: 2 * b.y - a.y,
        }
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let mut antennas: HashMap<Antenna, Vec<Location>> = HashMap::new();
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            if y == 0 {
                width = line.len() as isize;
            }
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let location = Location {
                        x: x as isize,
                        y: y as isize,
                    };
                    antennas
                        .entry(Antenna(c))
                        .and_modify(|locations| locations.push(location))
                        .or_insert(vec![location]);
                }
            }
        }
        Ok(Map {
            width,
            height: input.lines().count() as isize,
            antennas,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 14);

    test_example!("example1", Part2, 34);

    test_example!("example2", Part2, 9);
}
