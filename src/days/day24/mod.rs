use std::collections::BTreeMap;

use memoize::memoize;

use crate::*;

day! {
    Output = u64,
    Parsed = BTreeMap<&'static str, Wire>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Wire {
    Input(bool),
    LogicGate {
        input1: &'static str,
        operation: Operation,
        input2: &'static str,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        }
    }
}

impl Day {
    fn part1(wires: Parsed) -> Result<Output> {
        Ok(wires
            .keys()
            .filter(|wire_name| wire_name.starts_with("z"))
            .rev()
            .fold(0, |acc, wire_name| {
                acc * 2
                    + if get_output(wire_name, wires.clone()) {
                        1
                    } else {
                        0
                    }
            }))
    }

    fn part2(_parsed: Parsed) -> Result<Output> {
        Ok(0)
    }
}

#[memoize]
fn get_output(wire_name: &'static str, wires: BTreeMap<&'static str, Wire>) -> bool {
    match wires.get(wire_name).unwrap() {
        Wire::Input(value) => *value,
        Wire::LogicGate {
            input1,
            operation,
            input2,
        } => {
            let value1 = get_output(input1, wires.clone());
            let value2 = get_output(input2, wires.clone());
            match operation {
                Operation::And => value1 & value2,
                Operation::Or => value1 | value2,
                Operation::Xor => value1 ^ value2,
            }
        }
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let mut wires = BTreeMap::new();
        let (i, w) = input.split_once("\n\n").unwrap();
        for (wire_name, value) in i.lines().map(|line| line.split_once(": ").unwrap()) {
            wires.insert(wire_name, Wire::Input(value == "1"));
        }

        for split in w.lines().map(|line| line.split(" ").collect::<Vec<_>>()) {
            wires.insert(
                split[4],
                Wire::LogicGate {
                    input1: split[0],
                    operation: split[1].into(),
                    input2: split[2],
                },
            );
        }

        Ok(wires)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 4);

    test_example!("example2", Part1, 2024);

    test_example!("example1", Part2, 0);
}
