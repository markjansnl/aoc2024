use std::iter::zip;

use nom::{
    character::complete::{newline, space1, u32},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

use crate::*;

day! {
    Output = u32,
    Parsed = (Vec<Output>, Vec<Output>),
}

impl Day {
    #[inline]
    fn part1((mut left, mut right): Parsed) -> Result<Output> {
        left.sort_unstable();
        right.sort_unstable();

        Ok(zip(left, right).map(|(l, r)| l.abs_diff(r)).sum())
    }

    #[inline]
    fn part2((left, right): Parsed) -> Result<Output> {
        Ok(left
            .iter()
            .map(|l| right.iter().filter(|&r| r == l).count() as Output * l)
            .sum())
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::pairs)(input)?.1.into_iter().unzip())
    }

    #[inline]
    fn pairs(s: &'static str) -> IResult<Vec<(Output, Output)>> {
        separated_list1(newline, Self::pair)(s)
    }

    #[inline]
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
