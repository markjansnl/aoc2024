use std::collections::{BTreeMap, BTreeSet};

use crate::*;

day! {
    Output = String,
    Parsed = ParsedInput,
}

type NodeName = &'static str;

#[derive(Default)]
struct ParsedInput {
    nodes: BTreeSet<NodeName>,
    forward: BTreeMap<NodeName, BTreeSet<NodeName>>,
    all: BTreeMap<NodeName, BTreeSet<NodeName>>,
}

impl Day {
    fn part1(parsed: Parsed) -> Result<Output> {
        let mut interconnected = BTreeSet::new();
        for node1 in parsed
            .nodes
            .iter()
            .copied()
            .filter(|node_name| node_name.starts_with("t"))
        {
            let mut set = BTreeSet::new();
            set.insert(node1);

            if let Some(reachable) = parsed.all.get(node1) {
                for node2 in reachable {
                    if let Some(reachable_node2) = parsed.forward.get(node2) {
                        for node3 in reachable.iter() {
                            if reachable_node2.contains(node3) {
                                let mut set2 = set.clone();
                                set2.insert(node2);
                                set2.insert(node3);
                                interconnected.insert(set2);
                            }
                        }
                    }
                }
            }
        }
        Ok(interconnected.len().to_string())
    }

    fn part2(parsed: Parsed) -> Result<Output> {
        let path = parsed
            .nodes
            .iter()
            .copied()
            .map(|node| {
                let mut connected = BTreeSet::new();
                connected.insert(node);
                for (next, reachable) in &parsed.all {
                    if connected.intersection(reachable).count() == connected.len() {
                        connected.insert(*next);
                    }
                }
                connected
            })
            .max_by_key(|path| path.len())
            .unwrap();
        Ok(path.into_iter().collect::<Vec<_>>().join(","))
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let mut parsed = ParsedInput::default();
        for line in input.lines() {
            let (left, right) = line.split_once("-").unwrap();
            parsed.nodes.insert(left);
            parsed.nodes.insert(right);
            parsed.forward.entry(left).or_default().insert(right);
            parsed.all.entry(right).or_default().insert(left);
            parsed.all.entry(left).or_default().insert(right);
        }
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 7);

    test_example!("example1", Part2, "co,de,ka,ta");
}
