mod days;

use std::fmt::{self, Debug};

pub use anyhow::Result;
pub use Part::*;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub const DAYS: u8 = 25;

#[inline]
pub fn run(day: u8, part: Part) -> String {
    match day {
        1 => days::day01::Day::run(part),
        2 => days::day02::Day::run(part),
        3 => days::day03::Day::run(part),
        4 => days::day04::Day::run(part),
        5 => days::day05::Day::run(part),
        6 => days::day06::Day::run(part),
        7 => days::day07::Day::run(part),
        8 => days::day08::Day::run(part),
        9 => days::day09::Day::run(part),
        10 => days::day10::Day::run(part),
        11 => days::day11::Day::run(part),
        12 => days::day12::Day::run(part),
        13 => days::day13::Day::run(part),
        14 => days::day14::Day::run(part),
        15 => days::day15::Day::run(part),
        16 => days::day16::Day::run(part),
        17 => days::day17::Day::run(part),
        18 => days::day18::Day::run(part),
        19 => days::day19::Day::run(part),
        20 => days::day20::Day::run(part),
        21 => days::day21::Day::run(part),
        22 => days::day22::Day::run(part),
        23 => days::day23::Day::run(part),
        24 => days::day24::Day::run(part),
        25 => days::day25::Day::run(part),
        _ => unreachable!(),
    }
    .unwrap_or_else(|err| err.to_string())
}

#[inline]
pub fn bench_sample_size(day: u8) -> Option<usize> {
    match day {
        1 => days::day01::Day::bench_sample_size(),
        2 => days::day02::Day::bench_sample_size(),
        3 => days::day03::Day::bench_sample_size(),
        4 => days::day04::Day::bench_sample_size(),
        5 => days::day05::Day::bench_sample_size(),
        6 => days::day06::Day::bench_sample_size(),
        7 => days::day07::Day::bench_sample_size(),
        8 => days::day08::Day::bench_sample_size(),
        9 => days::day09::Day::bench_sample_size(),
        10 => days::day10::Day::bench_sample_size(),
        11 => days::day11::Day::bench_sample_size(),
        12 => days::day12::Day::bench_sample_size(),
        13 => days::day13::Day::bench_sample_size(),
        14 => days::day14::Day::bench_sample_size(),
        15 => days::day15::Day::bench_sample_size(),
        16 => days::day16::Day::bench_sample_size(),
        17 => days::day17::Day::bench_sample_size(),
        18 => days::day18::Day::bench_sample_size(),
        19 => days::day19::Day::bench_sample_size(),
        20 => days::day20::Day::bench_sample_size(),
        21 => days::day21::Day::bench_sample_size(),
        22 => days::day22::Day::bench_sample_size(),
        23 => days::day23::Day::bench_sample_size(),
        24 => days::day24::Day::bench_sample_size(),
        25 => days::day25::Day::bench_sample_size(),
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Part1,
    Part2,
}

impl Debug for Part {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part1 => write!(f, "Part 1"),
            Part2 => write!(f, "Part 2"),
        }
    }
}

impl From<u8> for Part {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1 => Part1,
            2 => Part2,
            _ => unreachable!(),
        }
    }
}

pub trait DayRunner {
    fn run(part: Part) -> Result<String>;

    fn bench_sample_size() -> Option<usize>;

    fn _run(input: &'static str, part: Part) -> Result<String>;
}

#[macro_export]
macro_rules! day {
    ($(Output = $output:ty,)? $(Parsed = $parsed:ty,)? $(bench_sample_size: $bench_sample_size:literal,)?) => {
        $( type Output = $output; )?
        $( type Parsed = $parsed; )?

        pub struct Day;
        struct Parser;

        impl DayRunner for Day {
            #[inline]
            fn run(part: Part) -> Result<String> {
                Self::_run(include_str!("input.txt"), part)
            }

            #[inline]
            fn bench_sample_size() -> Option<usize> {
                #[allow(unused_mut, unused_assignments)]
                let mut bench_sample_size = None;
                $( bench_sample_size = Some($bench_sample_size); )?
                bench_sample_size
            }

            #[inline]
            fn _run(input: &'static str, part: Part) -> Result<String> {
                let parsed = Parser::parse(input)?;
                Ok(match part {
                    Part1 => Self::part1(parsed)?.to_string(),
                    Part2 => Self::part2(parsed)?.to_string(),
                })
            }
        }
    };
}

#[macro_export]
macro_rules! test_example {
    ($example:literal, $part:ident, $expected:literal) => {
        paste::paste! {
            #[test]
            fn [< $example _ $part:lower >] () -> Result<()> {
                assert_eq!(Day::_run(include_str!(concat!($example, ".txt")), $part)?, $expected .to_string());
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! run {
    ($part:ident) => {
        paste::paste! {
            #[test]
            #[ignore]
            fn [< run_ $part:lower >] () -> Result<()> {
                println!("{:?}: {}", $part, Day::_run(include_str!("input.txt"), $part)?);
                Ok(())
            }
        }
    };
}
