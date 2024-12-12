use crate::*;

day! {
    Output = usize,
    Parsed = Puzzle,
}

struct Puzzle {
    lines: Vec<String>,
}

impl FromIterator<String> for Puzzle {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            lines: Vec::from_iter(iter),
        }
    }
}

impl Puzzle {
    fn count_xmas_part1(&self) -> Output {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == 'X')
                    .map(move |(x, _)| {
                        let mut count = 0;
                        for offset_y in -1..=1isize {
                            for offset_x in -1..=1isize {
                                if (offset_y != 0 || offset_x != 0)
                                    && self.check_mas(y as isize, x as isize, offset_y, offset_x)
                                {
                                    count += 1;
                                }
                            }
                        }
                        count
                    })
            })
            .sum()
    }

    fn check_mas(&self, y: isize, x: isize, offset_y: isize, offset_x: isize) -> bool {
        self.get(y + offset_y, x + offset_x) == "M"
            && self.get(y + offset_y * 2, x + offset_x * 2) == "A"
            && self.get(y + offset_y * 3, x + offset_x * 3) == "S"
    }

    fn get(&self, y: isize, x: isize) -> &str {
        if y >= 0 && y < self.lines.len() as isize && x >= 0 && x < self.lines[0].len() as isize {
            &self.lines[y as usize][x as usize..=x as usize]
        } else {
            "."
        }
    }

    fn count_xmas_part2(&self) -> Output {
        self.lines
            .iter()
            .enumerate()
            .skip(1)
            .take(self.lines.len() - 2)
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .skip(1)
                    .take(self.lines[0].len() - 2)
                    .filter(|(_, c)| *c == 'A')
                    .filter(move |(x, _)| {
                        let mut count_m = 0;
                        let mut count_s = 0;

                        self.check_m_s(y - 1, x - 1, &mut count_m, &mut count_s, 1)
                            && self.check_m_s(y + 1, x + 1, &mut count_m, &mut count_s, 1)
                            && self.check_m_s(y - 1, x + 1, &mut count_m, &mut count_s, 2)
                            && self.check_m_s(y + 1, x - 1, &mut count_m, &mut count_s, 2)
                    })
            })
            .count()
    }

    fn check_m_s(
        &self,
        y: usize,
        x: usize,
        count_m: &mut usize,
        count_s: &mut usize,
        max_count: usize,
    ) -> bool {
        match &self.lines[y][x..=x] {
            "M" => *count_m += 1,
            "S" => *count_s += 1,
            _ => return false,
        }
        *count_m <= max_count && *count_s <= max_count
    }
}

impl Day {
    fn part1(puzzle: Parsed) -> Result<Output> {
        Ok(puzzle.count_xmas_part1())
    }

    fn part2(puzzle: Parsed) -> Result<Output> {
        Ok(puzzle.count_xmas_part2())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(input.lines().map(String::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 18);

    test_example!("example1", Part2, 9);
}
