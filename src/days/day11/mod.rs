use memoize::memoize;

use nom::{
    bytes::complete::tag,
    character::complete::u64,
    combinator::all_consuming,
    multi::separated_list1,
};

use crate::*;

day! {
    Output = usize,
    Parsed = Vec<u64>,
}

impl Day {
    #[inline]
    fn part1(stones: Parsed) -> Result<Output> {
        Ok(stones
            .into_iter()
            .map(|stone| count_after_blinks(stone, 25))
            .sum())
    }

    #[inline]
    fn part2(stones: Parsed) -> Result<Output> {
        Ok(stones
            .into_iter()
            .map(|stone| count_after_blinks(stone, 75))
            .sum())
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
        let digits = format!("{stone}").len() as u32;
        if digits % 2 == 0 {
            let half_div = 10u64.pow(digits / 2);
            count_after_blinks(stone / half_div, blinks - 1)
                + count_after_blinks(stone % half_div, blinks - 1)
        } else {
            count_after_blinks(stone * 2024, blinks - 1)
        }
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::stones)(input)?.1)
    }

    #[inline]
    fn stones(s: &'static str) -> IResult<Parsed> {
        separated_list1(tag(" "), u64)(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 55312);
}
