use nom::{
    character::complete::{newline, space1, u32},
    combinator::{all_consuming, opt},
    multi::fold_many1,
    sequence::{pair, separated_pair},
};

use crate::*;

day! {
    Parsed = (Vec<u32>, Vec<u32>),
    Output = u32,
}

impl Day {
    fn part1((mut left, mut right): Parsed) -> Result<Output> {
        left.sort();
        right.sort();

        Ok(left
            .into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum())
    }

    fn part2((left, right): Parsed) -> Result<Output> {
        Ok(left
            .into_iter()
            .map(|l| right.iter().filter(|r| **r == l).count() as Output * l)
            .sum())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::pairs)(input)?.1)
    }

    fn pairs(s: &'static str) -> IResult<Parsed> {
        fold_many1(
            pair(Self::pair, opt(newline)),
            || (Vec::new(), Vec::new()),
            |(mut left, mut right), ((l, r), _)| {
                left.push(l);
                right.push(r);
                (left, right)
            },
        )(s)
    }

    fn pair(s: &'static str) -> IResult<(Output, Output)> {
        separated_pair(u32, space1, u32)(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 11);

    test_example!("example1", Part2, 31);
}
