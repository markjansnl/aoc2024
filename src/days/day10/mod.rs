use std::collections::BTreeSet;

use crate::*;

day! {
    Output = usize,
    Parsed = Vec<Vec<u8>>,
}

impl Day {
    fn part1(lines: Parsed) -> Result<Output> {
        let mut sum = 0;
        for (y, line) in lines.iter().enumerate() {
            for (x, &height) in line.iter().enumerate() {
                if height == 0 {
                    let mut trail_ends = BTreeSet::new();
                    Self::trail_score(&lines, x as isize, y as isize, 0, &mut trail_ends);
                    sum += trail_ends.len();
                }
            }
        }
        Ok(sum)
    }

    fn part2(lines: Parsed) -> Result<Output> {
        let mut sum = 0;
        for (y, line) in lines.iter().enumerate() {
            for (x, &height) in line.iter().enumerate() {
                if height == 0 {
                    sum += Self::trail_rating(&lines, x as isize, y as isize, 0);
                }
            }
        }
        Ok(sum)
    }

    fn trail_score(
        lines: &Parsed,
        x: isize,
        y: isize,
        height: u8,
        trail_ends: &mut BTreeSet<(isize, isize)>,
    ) {
        if height == 9 {
            trail_ends.insert((x, y));
        } else {
            if let Some(h) = Self::get(lines, x, y - 1) {
                if h == height + 1 {
                    Self::trail_score(lines, x, y - 1, height + 1, trail_ends);
                }
            }
            if let Some(h) = Self::get(lines, x, y + 1) {
                if h == height + 1 {
                    Self::trail_score(lines, x, y + 1, height + 1, trail_ends);
                }
            }
            if let Some(h) = Self::get(lines, x - 1, y) {
                if h == height + 1 {
                    Self::trail_score(lines, x - 1, y, height + 1, trail_ends);
                }
            }
            if let Some(h) = Self::get(lines, x + 1, y) {
                if h == height + 1 {
                    Self::trail_score(lines, x + 1, y, height + 1, trail_ends);
                }
            }
        }
    }

    fn trail_rating(lines: &Parsed, x: isize, y: isize, height: u8) -> usize {
        if height == 9 {
            1
        } else {
            let mut score = 0;
            if let Some(h) = Self::get(lines, x, y - 1) {
                if h == height + 1 {
                    score += Self::trail_rating(lines, x, y - 1, height + 1);
                }
            }
            if let Some(h) = Self::get(lines, x, y + 1) {
                if h == height + 1 {
                    score += Self::trail_rating(lines, x, y + 1, height + 1);
                }
            }
            if let Some(h) = Self::get(lines, x - 1, y) {
                if h == height + 1 {
                    score += Self::trail_rating(lines, x - 1, y, height + 1);
                }
            }
            if let Some(h) = Self::get(lines, x + 1, y) {
                if h == height + 1 {
                    score += Self::trail_rating(lines, x + 1, y, height + 1);
                }
            }
            score
        }
    }

    fn get(lines: &Parsed, x: isize, y: isize) -> Option<u8> {
        if y < 0 || y >= lines.len() as isize || x < 0 || x >= lines[0].len() as isize {
            None
        } else {
            Some(lines[y as usize][x as usize])
        }
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| if b == 46 { 100 } else { b - 48 })
                    .collect()
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 36);

    test_example!("example1", Part2, 81);
}
