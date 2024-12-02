use std::cmp::Ordering::*;

use nom::{
    character::complete::{newline, space1, u32},
    combinator::all_consuming,
    multi::separated_list1,
};

use crate::*;
use Ordering::*;

day! {
    Output = u32,
    Parsed = Vec<Vec<Output>>,
}

#[derive(PartialEq, Eq)]
enum Ordering {
    Increasing,
    Decreasing,
}

impl Day {
    #[inline]
    fn part1(reports: Parsed) -> Result<Output> {
        Ok(reports
            .into_iter()
            .filter(|report| Self::is_safe(report, false, None))
            .count() as Output)
    }

    #[inline]
    fn part2(reports: Parsed) -> Result<Output> {
        Ok(reports
            .into_iter()
            .filter(|report| Self::is_safe(report, true, None))
            .count() as Output)
    }

    #[inline]
    fn is_safe(report: &[Output], tolerance: bool, exclude: Option<usize>) -> bool {
        if exclude == Some(0) {
            return Self::is_safe(&report[1..], false, None);
        }

        let mut iter = report.iter().enumerate();
        let mut prev = iter.next().unwrap().1;
        let mut prev_ordering = None;

        for (idx, next) in iter {
            if Some(idx) == exclude {
                continue;
            }

            let next_ordering = Self::check_ordering(*prev, *next);
            if next_ordering.is_none()
                || (prev_ordering.is_some() && next_ordering != prev_ordering)
            {
                if tolerance {
                    return Self::is_safe(report, false, Some(idx))
                        || Self::is_safe(report, false, Some(idx - 1))
                        || (idx >= 2 && Self::is_safe(report, false, Some(idx - 2)));
                } else {
                    return false;
                }
            } else {
                prev = next;
                prev_ordering = next_ordering;
            }
        }
        true
    }

    #[inline]
    fn check_ordering(prev: Output, next: Output) -> Option<Ordering> {
        match prev.cmp(&next) {
            Equal => None,
            Less => (next - prev <= 3).then_some(Increasing),
            Greater => (prev - next <= 3).then_some(Decreasing),
        }
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::reports)(input)?.1)
    }

    #[inline]
    fn reports(s: &'static str) -> IResult<Parsed> {
        separated_list1(newline, Self::report)(s)
    }

    #[inline]
    fn report(s: &'static str) -> IResult<Vec<Output>> {
        separated_list1(space1, u32)(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 2);

    test_example!("example1", Part2, 4);
}
