use std::collections::BTreeSet;

use crate::*;

use Direction::*;

day! {
    Output = usize,
    Parsed = Map,
    bench_sample_size: 50,
}

struct Map {
    plots: &'static [u8],
    width: usize,
    width_incl_newline: usize,
    heigth: usize,
}

type FencesPart1 = usize;
type FencesPart2 = BTreeSet<FenceLocation>;

#[derive(Default)]
struct Region<F: Fences> {
    area: usize,
    fences: F,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct FenceLocation {
    direction: Direction,
    a: isize,
    b: isize,
}

struct LocationSet<'a> {
    map: &'a Map,
    locations: [bool; 20_000],
}

impl Location {
    fn next(&self, direction: Direction) -> Location {
        match direction {
            Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Down => Self {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

impl<'a> LocationSet<'a> {
    fn with_map(map: &'a Map) -> Self {
        LocationSet {
            map,
            locations: [false; 20_000],
        }
    }

    fn insert(&mut self, location: Location) {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize] =
            true;
    }

    fn contains(&mut self, location: Location) -> bool {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize]
    }

    fn remove(&mut self, location: Location) {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize] =
            false;
    }

    fn first(&mut self) -> Option<Location> {
        self.locations
            .iter()
            .enumerate()
            .find(|(_, contains)| **contains)
            .map(|(idx, _)| Location {
                x: (idx % self.map.width_incl_newline) as isize,
                y: (idx / self.map.width_incl_newline) as isize,
            })
    }
}

trait Fences: Default {
    fn insert(&mut self, location: Location, direction: Direction);
    fn len(&self) -> usize;
    fn iter(&self) -> impl Iterator<Item = &FenceLocation>;
}

impl Fences for FencesPart1 {
    fn insert(&mut self, _location: Location, _direction: Direction) {
        *self += 1;
    }

    fn len(&self) -> usize {
        *self
    }

    fn iter(&self) -> impl Iterator<Item = &FenceLocation> {
        std::vec::IntoIter::<_>::default()
    }
}

impl Fences for FencesPart2 {
    fn insert(&mut self, location: Location, direction: Direction) {
        match direction {
            Left | Right => self.insert(FenceLocation {
                direction,
                a: location.x,
                b: location.y,
            }),
            Up | Down => self.insert(FenceLocation {
                direction,
                a: location.y,
                b: location.x,
            }),
        };
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&self) -> impl Iterator<Item = &FenceLocation> {
        self.iter()
    }
}

impl<F: Fences> Region<F> {
    fn total_price(&self) -> Output {
        self.area * self.fences.len()
    }

    fn bulk_discount(&self) -> Output {
        self.area * self.count_sides()
    }

    fn count_sides(&self) -> Output {
        let mut fences_iter = self.fences.iter();
        let mut prev = fences_iter.next().unwrap();
        let mut sides = 1;
        for next in fences_iter {
            if next.direction != prev.direction || next.a != prev.a || next.b != prev.b + 1 {
                sides += 1;
            }
            prev = next;
        }
        sides
    }
}

impl Map {
    fn get(&self, location: Location) -> Option<u8> {
        if location.y < 0
            || location.y as usize >= self.heigth
            || location.x < 0
            || location.x as usize >= self.width
        {
            None
        } else {
            Some(self.plots[location.y as usize * self.width_incl_newline + location.x as usize])
        }
    }

    fn regions<F: Fences>(&self) -> Vec<Region<F>> {
        let mut visited = LocationSet::with_map(self);
        let mut visit = LocationSet::with_map(self);
        let mut regions = vec![];

        visit.insert(Location::default());
        while let Some(location) = visit.first() {
            let plant = self.get(location).unwrap();
            let mut region = Region::default();
            self.find_regions(location, plant, &mut region, &mut visited, &mut visit);
            regions.push(region);
        }

        regions
    }

    fn find_regions<F: Fences>(
        &self,
        location: Location,
        plant: u8,
        region: &mut Region<F>,
        visited: &mut LocationSet,
        visit: &mut LocationSet,
    ) {
        visit.remove(location);
        visited.insert(location);
        region.area += 1;

        for direction in [Right, Down, Left, Up] {
            let next_location = location.next(direction);
            if let Some(next_plant) = self.get(next_location) {
                if next_plant == plant {
                    if !visited.contains(next_location) {
                        self.find_regions(next_location, plant, region, visited, visit);
                    }
                } else {
                    region.fences.insert(location, direction);
                    if !visited.contains(next_location) {
                        visit.insert(next_location);
                    }
                }
            } else {
                region.fences.insert(location, direction);
            }
        }
    }
}

impl Day {
    fn part1(map: Parsed) -> Result<Output> {
        Ok(map
            .regions()
            .into_iter()
            .map(|region: Region<FencesPart1>| region.total_price())
            .sum())
    }

    fn part2(map: Parsed) -> Result<Output> {
        Ok(map
            .regions()
            .into_iter()
            .map(|region: Region<FencesPart2>| region.bulk_discount())
            .sum())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let width = input.find('\n').unwrap();
        let width_incl_newline = width + 1;
        Ok(Map {
            plots: input.as_bytes(),
            width,
            width_incl_newline,
            heigth: (input.len() + 1) / width_incl_newline,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 140);

    test_example!("example2", Part1, 772);

    test_example!("example3", Part1, 1930);

    test_example!("example1", Part2, 80);

    test_example!("example2", Part2, 436);

    test_example!("example3", Part2, 1206);

    test_example!("example4", Part2, 236);

    test_example!("example5", Part2, 368);
}
