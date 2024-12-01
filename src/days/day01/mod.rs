use std::cmp::Ordering::*;
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
    fn part1((left, right): Parsed) -> Result<Output> {
        Ok(zip(left, right).map(|(l, r)| l.abs_diff(r)).sum())
    }

    #[inline]
    fn part2((left, right): Parsed) -> Result<Output> {
        let mut left_iter = left.into_iter().peekable();
        let mut right_iter = right.into_iter().peekable();
        let mut sum = 0;

        while let (Some(&l), Some(&r)) = (left_iter.peek(), right_iter.peek()) {
            match l.cmp(&r) {
                Less => {
                    left_iter.next();
                }
                Greater => {
                    right_iter.next();
                }
                Equal => {
                    let mut count_l = 0;
                    let mut count_r = 0;
                    while left_iter.next_if_eq(&l).is_some() {
                        count_l += 1;
                    }
                    while right_iter.next_if_eq(&r).is_some() {
                        count_r += 1;
                    }
                    sum += l * count_l * count_r;
                }
            }
        }

        Ok(sum)
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        let pairs = all_consuming(Self::pairs)(input)?.1;
        let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        left.sort_unstable();
        right.sort_unstable();
        Ok((left, right))
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
