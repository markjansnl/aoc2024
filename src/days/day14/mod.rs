use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::*;

day! {
    Output = i32,
    Parsed = Robots,
}

struct Robots {
    robots: Vec<Robot>,
    width: Output,
    height: Output,
}

struct Robot {
    position: Position,
    velocity: Position,
}

struct Position {
    x: Output,
    y: Output,
}

impl From<Vec<Robot>> for Robots {
    fn from(robots: Vec<Robot>) -> Self {
        let (width, height) = if robots.len() == 12 {
            (11, 7)
        } else {
            (101, 103)
        };

        Self {
            robots,
            width,
            height,
        }
    }
}

impl Day {
    fn part1(robots: Parsed) -> Result<Output> {
        Ok(Self::safety_factor(&robots, 100))
    }

    fn part2(robots: Parsed) -> Result<Output> {
        Ok((1..6_500)
            .map(|i| (i, Self::safety_factor(&robots, i)))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0)
    }

    fn safety_factor(robots: &Parsed, i: Output) -> Output {
        robots
            .robots
            .iter()
            .map(|robot| Position {
                x: (robot.position.x + i * robot.velocity.x).rem_euclid(robots.width),
                y: (robot.position.y + i * robot.velocity.y).rem_euclid(robots.height),
            })
            .map(|position| {
                (
                    position.x.cmp(&(robots.width / 2)),
                    position.y.cmp(&(robots.height / 2)),
                )
            })
            .fold([0; 4], |mut quadrants, ordering| {
                match ordering {
                    (Ordering::Less, Ordering::Less) => quadrants[0] += 1,
                    (Ordering::Less, Ordering::Greater) => quadrants[1] += 1,
                    (Ordering::Greater, Ordering::Less) => quadrants[2] += 1,
                    (Ordering::Greater, Ordering::Greater) => quadrants[3] += 1,
                    _ => {}
                }
                quadrants
            })
            .into_iter()
            .product()
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::robots)(input)?.1)
    }

    fn robots(s: &'static str) -> IResult<Parsed> {
        map(separated_list1(newline, Self::robot), Robots::from)(s)
    }

    fn robot(s: &'static str) -> IResult<Robot> {
        map(
            separated_pair(
                preceded(tag("p="), Self::position),
                tag(" v="),
                Self::position,
            ),
            |(position, velocity)| Robot { position, velocity },
        )(s)
    }

    fn position(s: &'static str) -> IResult<Position> {
        map(separated_pair(i32, tag(","), i32), |(x, y)| Position {
            x: x as Output,
            y: y as Output,
        })(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 12);
}
