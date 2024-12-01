mod days;

use std::fmt::{self, Debug};

pub use anyhow::Result;
pub use Part::*;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub const DAYS: u8 = 2;

#[inline]
pub fn run(day: u8, part: Part) -> String {
    match day {
        1 => days::day01::Day::run(part),
        2 => days::day02::Day::run(part),
        _ => unreachable!(),
    }
    .unwrap_or_else(|err| err.to_string())
}

#[inline]
pub fn bench_sample_size(day: u8) -> Option<usize> {
    match day {
        1 => days::day01::Day::bench_sample_size(),
        2 => days::day02::Day::bench_sample_size(),
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy)]
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
    ($($typ:ident = $ty:ty),+, $(bench_sample_size: $bench_sample_size:literal,)?) => {
        $( type $typ = $ty; )+

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
