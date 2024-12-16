use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, tuple},
};

use crate::*;

day! {
    Output = isize,
    Parsed = Vec<Machine>,
}

struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

struct Position {
    x: Output,
    y: Output,
}

impl Machine {
    fn fewest_tokens(self) -> Output {
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        // A = (67 * 8400 - 22 * 5400) / (67 * 94 – 22 * 34)

        let a_teller = self.button_b.y * self.prize.x - self.button_b.x * self.prize.y;
        let a_noemer = self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y;
        if a_teller % a_noemer == 0 {
            let a = a_teller / a_noemer;
            let b_teller = self.prize.x - a * self.button_a.x;
            let b_noemer = self.button_b.x;
            if b_teller % b_noemer == 0 {
                let b = b_teller / b_noemer;
                return 3 * a + b;
            }
        }
        0
    }

    fn into_part2(self) -> Self {
        Machine {
            button_a: self.button_a,
            button_b: self.button_b,
            prize: Position {
                x: self.prize.x + 10_000_000_000_000,
                y: self.prize.y + 10_000_000_000_000,
            },
        }
    }
}

impl Day {
    fn part1(machines: Parsed) -> Result<Output> {
        Ok(machines.into_iter().map(Machine::fewest_tokens).sum())
    }

    fn part2(machines: Parsed) -> Result<Output> {
        Ok(machines
            .into_iter()
            .map(Machine::into_part2)
            .map(Machine::fewest_tokens)
            .sum())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::machines)(input)?.1)
    }

    fn machines(s: &'static str) -> IResult<Parsed> {
        separated_list1(pair(newline, newline), Self::machine)(s)
    }

    fn machine(s: &'static str) -> IResult<Machine> {
        map(
            tuple((Self::button, newline, Self::button, newline, Self::prize)),
            |(button_a, _, button_b, _, prize)| Machine {
                button_a,
                button_b,
                prize,
            },
        )(s)
    }

    fn button(s: &'static str) -> IResult<Position> {
        map(
            separated_pair(
                preceded(alt((tag("Button A: X+"), tag("Button B: X+"))), u64),
                tag(", Y+"),
                u64,
            ),
            |(x, y)| Position {
                x: x as Output,
                y: y as Output,
            },
        )(s)
    }

    fn prize(s: &'static str) -> IResult<Position> {
        map(
            separated_pair(preceded(tag("Prize: X="), u64), tag(", Y="), u64),
            |(x, y)| Position {
                x: x as Output,
                y: y as Output,
            },
        )(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 480);

    test_example!("example1", Part2, 875_318_608_908_isize);
}
