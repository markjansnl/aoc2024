use std::collections::BTreeSet;

use crate::*;

use Direction::*;

day! {
    Output = usize,
    Parsed = Map,
}

struct Map {
    plots: &'static [u8],
    width: usize,
    width_incl_newline: usize,
    heigth: usize,
}

#[derive(Default)]
struct Region {
    area: usize,
    fences: BTreeSet<FenceLocation>,
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
    #[inline]
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
    #[inline]
    fn with_map(map: &'a Map) -> Self {
        LocationSet {
            map,
            locations: [false; 20_000],
        }
    }

    #[inline]
    fn insert(&mut self, location: Location) {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize] =
            true;
    }

    #[inline]
    fn contains(&mut self, location: Location) -> bool {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize]
    }

    #[inline]
    fn remove(&mut self, location: Location) {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize] =
            false;
    }

    #[inline]
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

impl Region {
    #[inline]
    fn insert_fence(&mut self, location: Location, direction: Direction) {
        match direction {
            Left | Right => self.fences.insert(FenceLocation {
                direction,
                a: location.x,
                b: location.y,
            }),
            Up | Down => self.fences.insert(FenceLocation {
                direction,
                a: location.y,
                b: location.x,
            }),
        };
    }

    #[inline]
    fn total_price(&self) -> Output {
        self.area * self.fences.len()
    }

    #[inline]
    fn bulk_discount(&self) -> Output {
        self.area * self.count_sides()
    }

    #[inline]
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
    #[inline]
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

    #[inline]
    fn regions(&self) -> Vec<Region> {
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

    #[inline]
    fn find_regions(
        &self,
        location: Location,
        plant: u8,
        region: &mut Region,
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
                    region.insert_fence(location, direction);
                    if !visited.contains(next_location) {
                        visit.insert(next_location);
                    }
                }
            } else {
                region.insert_fence(location, direction);
            }
        }
    }
}

impl Day {
    #[inline]
    fn part1(map: Parsed) -> Result<Output> {
        Ok(map
            .regions()
            .into_iter()
            .map(|region| region.total_price())
            .sum())
    }

    #[inline]
    fn part2(map: Parsed) -> Result<Output> {
        Ok(map
            .regions()
            .into_iter()
            .map(|region| region.bulk_discount())
            .sum())
    }
}

impl Parser {
    #[inline]
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
