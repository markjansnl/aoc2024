use memoize::memoize;

use crate::*;

day! {
    Output = usize,
    Parsed = Input,
}

struct Input {
    patterns: Vec<&'static str>,
    designs: Vec<&'static str>,
}

impl Day {
    fn part1(parsed: Parsed) -> Result<Output> {
        Ok(parsed
            .designs
            .into_iter()
            .filter(|design| is_possible(design, parsed.patterns.clone()))
            .count())
    }

    fn part2(parsed: Parsed) -> Result<Output> {
        Ok(parsed
            .designs
            .into_iter()
            .map(|design| arrangements(design, parsed.patterns.clone()))
            .sum())
    }
}

#[memoize]
fn is_possible(design: &'static str, patterns: Vec<&'static str>) -> bool {
    patterns.iter().any(|pattern| {
        design == *pattern
            || (design.starts_with(pattern)
                && is_possible(&design[pattern.len()..], patterns.clone()))
    })
}

#[memoize]
fn arrangements(design: &'static str, patterns: Vec<&'static str>) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            if design == *pattern {
                1
            } else if let Some(tail) = design.strip_prefix(pattern) {
                arrangements(tail, patterns.clone())
            } else {
                0
            }
        })
        .sum()
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let (patterns, designs) = input.split_once("\n\n").unwrap();
        Ok(Input {
            patterns: patterns.split(", ").collect(),
            designs: designs.lines().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 6);

    test_example!("example1", Part2, 16);
}
