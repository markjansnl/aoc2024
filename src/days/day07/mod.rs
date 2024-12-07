use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::prelude::*;

use crate::*;

day! {
    Output = Number,
    Parsed = Calibration,
}

type Number = u64;
type Calibration = Vec<CalibrationEquation>;

struct CalibrationEquation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Day {
    #[inline]
    fn part1(calibration: Parsed) -> Result<Output> {
        Ok(calibration
            .into_iter()
            .filter(|calibration| {
                Self::check_part1(
                    calibration.numbers[0],
                    &calibration.numbers[1..],
                    calibration.test_value,
                )
            })
            .map(|calibration| calibration.test_value)
            .sum())
    }

    #[inline]
    fn part2(calibration: Parsed) -> Result<Output> {
        Ok(calibration
            .into_par_iter()
            .filter(|calibration| {
                Self::check_part2(
                    calibration.numbers[0],
                    &calibration.numbers[1..],
                    calibration.test_value,
                )
            })
            .map(|calibration| calibration.test_value)
            .sum())
    }

    #[inline]
    fn check_part1(evaluated: Number, numbers: &[Number], test_value: Number) -> bool {
        if numbers.is_empty() {
            evaluated == test_value
        } else if evaluated > test_value {
            false
        } else {
            Self::check_part1(evaluated + numbers[0], &numbers[1..], test_value)
                || Self::check_part1(evaluated * numbers[0], &numbers[1..], test_value)
        }
    }

    #[inline]
    fn check_part2(evaluated: Number, numbers: &[Number], test_value: Number) -> bool {
        if numbers.is_empty() {
            evaluated == test_value
        } else if evaluated > test_value {
            false
        } else {
            Self::check_part2(evaluated + numbers[0], &numbers[1..], test_value)
                || Self::check_part2(evaluated * numbers[0], &numbers[1..], test_value)
                || Self::check_part2(
                    format!("{}{}", evaluated, numbers[0]).parse().unwrap(),
                    &numbers[1..],
                    test_value,
                )
        }
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::calibration)(input)?.1)
    }

    #[inline]
    fn calibration(s: &'static str) -> IResult<Parsed> {
        separated_list1(newline, Self::calibration_equation)(s)
    }

    #[inline]
    fn calibration_equation(s: &'static str) -> IResult<CalibrationEquation> {
        map(
            separated_pair(u64, tag(": "), Self::numbers),
            |(test_value, numbers)| CalibrationEquation {
                test_value,
                numbers,
            },
        )(s)
    }

    #[inline]
    fn numbers(s: &'static str) -> IResult<Vec<u64>> {
        separated_list1(tag(" "), u64)(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 3749);

    test_example!("example1", Part2, 11387);
}
