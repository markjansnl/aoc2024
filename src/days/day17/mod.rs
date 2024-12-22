use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
};

use crate::*;

day! {
    Output = String,
    Parsed = Device,
}

type Number = usize;

struct Device {
    registers: [Number; 3],
    program: Vec<Number>,
    ip: usize,
}

impl Device {
    fn run(&mut self) -> Vec<Number> {
        let mut output = Vec::new();
        while let Some((instruction, operand)) = self.get_operation() {
            match instruction {
                0 => {
                    self.registers[0] /= (2 as Number).pow(self.combo(operand) as u32);
                }
                1 => {
                    self.registers[1] ^= operand;
                }
                2 => {
                    self.registers[1] = self.combo(operand) % 8;
                }
                3 => {
                    if self.registers[0] != 0 {
                        self.ip = operand;
                    }
                }
                4 => {
                    self.registers[1] ^= self.registers[2];
                }
                5 => output.push(self.combo(operand) % 8),
                6 => {
                    self.registers[1] =
                        self.registers[0] / (2 as Number).pow(self.combo(operand) as u32);
                }
                7 => {
                    self.registers[2] =
                        self.registers[0] / (2 as Number).pow(self.combo(operand) as u32);
                }
                _ => unreachable!(),
            }
        }
        output
    }

    fn get_operation(&mut self) -> Option<(Number, Number)> {
        if self.ip <= self.program.len() - 2 {
            let ip = self.ip;
            self.ip += 2;

            Some((self.program[ip], self.program[ip + 1]))
        } else {
            None
        }
    }

    fn combo(&self, operand: Number) -> Number {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand - 4],
            _ => unreachable!(),
        }
    }
}

impl Day {
    fn part1(mut device: Parsed) -> Result<Output> {
        Ok(device
            .run()
            .iter()
            .map(Number::to_string)
            .collect::<Vec<_>>()
            .join(","))
    }

    fn part2(_parsed: Parsed) -> Result<Output> {
        Ok("".to_string())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::device)(input)?.1)
    }

    fn device(s: &'static str) -> IResult<Parsed> {
        map(
            separated_pair(Self::registers, newline, Self::program),
            |(registers, program)| Device {
                registers,
                program,
                ip: 0,
            },
        )(s)
    }

    fn registers(s: &'static str) -> IResult<[Number; 3]> {
        map(
            tuple((Self::register, Self::register, Self::register)),
            |(a, b, c)| [a, b, c],
        )(s)
    }

    fn register(s: &'static str) -> IResult<Number> {
        terminated(
            preceded(
                tuple((
                    tag("Register "),
                    alt((tag("A"), tag("B"), tag("C"))),
                    tag(": "),
                )),
                map(u64, |n| n as Number),
            ),
            newline,
        )(s)
    }

    fn program(s: &'static str) -> IResult<Vec<Number>> {
        preceded(
            tag("Program: "),
            separated_list1(tag(","), map(u64, |n| n as Number)),
        )(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    // #[test]
    // fn operation1() {
    //     let mut device = Device {
    //         registers: [0, 0, 9],
    //         program: vec![2, 6],
    //         ip: 0,
    //     };
    //     device.run();
    //     assert_eq!(device.registers[1], 1);
    // }

    test_example!("example1", Part1, "4,6,3,5,6,3,5,2,1,0");

    test_example!("example2", Part1, 0);
}
