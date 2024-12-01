use nom::{character::complete::i32, combinator::all_consuming};

use crate::*;

day! {
    Parsed = i32,
    Output = i32,
}

impl Day {
    fn part1(parsed: Parsed) -> Result<Output> {
        Ok(parsed)
    }

    fn part2(_parsed: Parsed) -> Result<Output> {
        Ok(0)
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::integer)(input)?.1)
    }

    fn integer(s: &'static str) -> IResult<i32> {
        i32(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 0);

    test_example!("example1", Part2, 0);
}
