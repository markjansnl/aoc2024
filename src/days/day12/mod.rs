use crate::*;

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
    perimeter: usize,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    #[inline]
    fn left(&self) -> Location {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    #[inline]
    fn right(&self) -> Location {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(&self) -> Location {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    #[inline]
    fn down(&self) -> Location {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

struct LocationSet<'a> {
    map: &'a Map,
    locations: [bool; 20_000],
}

impl<'a> LocationSet<'a> {
    #[inline]
    fn with_map(map: &'a Map) -> Self {
        LocationSet { map, locations: [false; 20_000] }
    }

    #[inline]
    fn insert(&mut self, location: Location) {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize] = true;
    }

    #[inline]
    fn contains(&mut self, location: Location) -> bool {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize]
    }

    #[inline]
    fn remove(&mut self, location: Location) {
        self.locations[location.y as usize * self.map.width_incl_newline + location.x as usize] = false;
    }

    #[inline]
    fn first(&mut self) -> Option<Location> {
        self.locations.iter().enumerate().find(|(_, contains)| **contains).map(|(idx, _)| Location {
            x: (idx % self.map.width_incl_newline) as isize,
            y: (idx / self.map.width_incl_newline) as isize,
        })
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
        let mut visited = LocationSet::with_map(&self);
        let mut visit = LocationSet::with_map(&self);
        let mut regions = vec![];

        visit.insert(Location::default());
        while let Some(location) = visit.first() {
            let mut region = Region::default();
            self.find_regions(
                location,
                self.get(location).unwrap(),
                &mut region,
                &mut visited,
                &mut visit,
            );
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

        for next_location in [
            location.right(),
            location.down(),
            location.left(),
            location.up(),
        ] {
            if let Some(next_plant) = self.get(next_location) {
                if next_plant == plant {
                    if !visited.contains(next_location) {
                        self.find_regions(next_location, plant, region, visited, visit);
                    }
                } else {
                    region.perimeter += 1;
                    if !visited.contains(next_location) {
                        visit.insert(next_location);
                    }
                }
            } else {
                region.perimeter += 1;
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
            .map(|Region { area, perimeter }| area * perimeter)
            .sum())
    }

    #[inline]
    fn part2(_parsed: Parsed) -> Result<Output> {
        Ok(0)
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

    test_example!("example1", Part2, 0);
}
