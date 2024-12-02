use std::cmp::Ordering;

use nom::{
    character::complete::{newline, space1, u32},
    combinator::all_consuming,
    multi::separated_list1,
};

use crate::*;

day! {
    Output = u32,
    Parsed = Vec<Vec<Output>>,
}

impl Day {
    #[inline]
    fn part1(reports: Parsed) -> Result<Output> {
        Ok(reports.into_iter().filter(Self::is_safe).count() as Output)
    }

    #[inline]
    fn part2(_reports: Parsed) -> Result<Output> {
        Ok(0)
    }

    fn is_safe(report: &Vec<Output>) -> bool {
        let mut windows = report.windows(2);

        let first = Self::check_ordering(
            windows
                .next()
                .expect("There must be one window in the report"),
        );
        if first.is_none() {
            return false;
        }

        while let Some(window) = windows.next() {
            let ordering = Self::check_ordering(window);
            if ordering.is_none() || ordering != first {
                return false;
            }
        }

        true
    }

    fn check_ordering(window: &[Output]) -> Option<Ordering> {
        match window[0].cmp(&window[1]) {
            Ordering::Equal => None,
            Ordering::Less => (window[1] - window[0] <= 3).then_some(Ordering::Less),
            Ordering::Greater => (window[0] - window[1] <= 3).then_some(Ordering::Greater),
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
