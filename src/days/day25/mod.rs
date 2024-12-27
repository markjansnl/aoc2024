use crate::*;

day! {
    Output = usize,
    Parsed = Vec<Schematic>,
}

type PinHeights = [u8; 5];

enum Schematic {
    Key(PinHeights),
    Lock(PinHeights),
}

impl Schematic {
    fn is_key(&self) -> bool {
        matches!(self, Schematic::Key(_))
    }

    fn is_lock(&self) -> bool {
        matches!(self, Schematic::Lock(_))
    }

    fn fits(&self, rhs: &Self) -> bool {
        if let Schematic::Lock(lock) = self {
            if let Schematic::Key(key) = rhs {
                for x in 0..5 {
                    if lock[x] + key[x] > 5 {
                        return false;
                    }
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        true
    }
}

impl Day {
    fn part1(schematics: Parsed) -> Result<Output> {
        let mut count = 0;
        for key in schematics.iter().filter(|s| s.is_key()) {
            for lock in schematics.iter().filter(|s| s.is_lock()) {
                if lock.fits(key) {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    fn part2(_parsed: Parsed) -> Result<Output> {
        Ok(0)
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(input
            .split("\n\n")
            .map(|schematic| {
                let mut pin_heights = PinHeights::default();
                for line in schematic.lines().skip(1).take(5) {
                    for (x, c) in line.chars().enumerate() {
                        if c == '#' {
                            pin_heights[x] += 1;
                        }
                    }
                }
                let first = schematic.chars().next().unwrap();
                if first == '#' {
                    Schematic::Lock(pin_heights)
                } else {
                    Schematic::Key(pin_heights)
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    test_example!("example1", Part1, 3);
}
