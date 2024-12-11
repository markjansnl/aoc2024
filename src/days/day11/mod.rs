use std::str::Bytes;

use memoize::memoize;

use crate::*;

day! {
    Output = usize,
    Parsed = StoneIter,
}

impl Day {
    #[inline]
    fn part1(stones: Parsed) -> Result<Output> {
        Ok(stones.map(|stone| count_after_blinks(stone, 25)).sum())
    }

    #[inline]
    fn part2(stones: Parsed) -> Result<Output> {
        Ok(stones.map(|stone| count_after_blinks(stone, 75)).sum())
    }
}

#[memoize]
#[inline]
fn count_after_blinks(stone: u64, blinks: u8) -> usize {
    if blinks == 0 {
        1
    } else if stone == 0 {
        count_after_blinks(1, blinks - 1)
    } else {
        let digits = ((stone + 1) as f64).log10().ceil() as u32;
        if digits % 2 == 0 {
            let half_div = 10u64.pow(digits / 2);
            count_after_blinks(stone / half_div, blinks - 1)
                + count_after_blinks(stone % half_div, blinks - 1)
        } else {
            count_after_blinks(stone * 2024, blinks - 1)
        }
    }
}

struct StoneIter {
    bytes: Option<Bytes<'static>>,
    number: u64,
}

impl From<&'static str> for StoneIter {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self {
            bytes: Some(value.bytes()),
            number: 0,
        }
    }
}

impl Iterator for StoneIter {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(bytes) = &mut self.bytes {
            while let Some(b) = bytes.next() {
                if b == 32 {
                    let next = self.number;
                    self.number = 0;
                    return Some(next);
                }
                self.number = 10 * self.number + (b - 48) as u64
            }
            self.bytes = None;
            Some(self.number)
        } else {
            None
        }
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(StoneIter::from(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 55312);
}
