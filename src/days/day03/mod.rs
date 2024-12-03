use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    combinator::{all_consuming, map, map_res},
    error::ErrorKind,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
};

use crate::*;

day! {
    Output = u32,
    Parsed = Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Mul(Output, Output),
    Do,
    Dont,
    Invalid,
}

impl Day {
    #[inline]
    fn part1(parsed: Parsed) -> Result<Output> {
        Ok(parsed
            .into_iter()
            .filter_map(|instruction| match instruction {
                Instruction::Mul(a, b) => Some(a * b),
                _ => None,
            })
            .sum())
    }

    #[inline]
    fn part2(parsed: Parsed) -> Result<Output> {
        Ok(parsed
            .into_iter()
            .fold((0, true), |(sum, enabled), instruction| match instruction {
                Instruction::Mul(a, b) => (sum + if enabled { a * b } else { 0 }, enabled),
                Instruction::Do => (sum, true),
                Instruction::Dont => (sum, false),
                _ => (sum, enabled),
            })
            .0)
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::instructions)(input)?.1)
    }

    #[inline]
    fn instructions(s: &'static str) -> IResult<Parsed> {
        many1(Self::instruction)(s)
    }

    #[inline]
    fn instruction(s: &'static str) -> IResult<Instruction> {
        alt((Self::mul, Self::parse_do, Self::dont, Self::invalid))(s)
    }

    #[inline]
    fn mul(s: &'static str) -> IResult<Instruction> {
        map(
            preceded(
                tag("mul("),
                terminated(
                    separated_pair(Self::u32_max_999, tag(","), Self::u32_max_999),
                    tag(")"),
                ),
            ),
            |(a, b)| Instruction::Mul(a, b),
        )(s)
    }

    #[inline]
    fn parse_do(s: &'static str) -> IResult<Instruction> {
        map(tag("do()"), |_| Instruction::Do)(s)
    }

    #[inline]
    fn dont(s: &'static str) -> IResult<Instruction> {
        map(tag("don't()"), |_| Instruction::Dont)(s)
    }

    #[inline]
    fn invalid(s: &'static str) -> IResult<Instruction> {
        map(anychar, |_| Instruction::Invalid)(s)
    }

    #[inline]
    fn u32_max_999(s: &'static str) -> IResult<Output> {
        map_res(u32, |n| {
            if n <= 999 {
                Ok(n)
            } else {
                Err(nom::Err::Error(("Number too large", ErrorKind::Digit)))
            }
        })(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 161);

    test_example!("example2", Part2, 48);
}
