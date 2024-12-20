use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u8},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};

use crate::*;

day! {
    Output = usize,
    Parsed = Input,
}

type Section = Vec<PageNumber>;
type PageNumber = usize;

struct Input {
    ordering_rules: Vec<OrderingRule>,
    sections: Vec<Section>,
}

struct OrderingRule {
    before: PageNumber,
    after: PageNumber,
}

struct OrderingRulesSet {
    ordering_rules_set: [bool; 10_000],
}

impl From<&Vec<OrderingRule>> for OrderingRulesSet {
    fn from(ordering_rules: &Vec<OrderingRule>) -> Self {
        let mut ordering_rules_set = [false; 10_000];
        for ordering_rule in ordering_rules {
            ordering_rules_set[ordering_rule.before * 100 + ordering_rule.after] = true;
        }
        Self { ordering_rules_set }
    }
}

impl OrderingRulesSet {
    fn contains(&self, before: &PageNumber, after: &PageNumber) -> bool {
        self.ordering_rules_set[before * 100 + after]
    }
}

trait MiddlePageNumber {
    fn middle_page_number(&self) -> PageNumber;
}

impl MiddlePageNumber for Vec<PageNumber> {
    fn middle_page_number(&self) -> PageNumber {
        self[(self.len() - 1) / 2]
    }
}

trait SectionOrdering {
    fn correctly_ordered(&self, ordering_rules: &[OrderingRule]) -> bool;
    fn reorder(&self, ordering_rules: &OrderingRulesSet) -> Self;
}

impl SectionOrdering for Section {
    fn correctly_ordered(&self, ordering_rules: &[OrderingRule]) -> bool {
        let mut indices = [None; 100];
        for (idx, &page) in self.iter().enumerate() {
            indices[page] = Some(idx);
        }
        ordering_rules.iter().all(|OrderingRule { before, after }| {
            match (indices[*before], indices[*after]) {
                (Some(idx_before), Some(idx_after)) => idx_before < idx_after,
                _ => true,
            }
        })
    }

    fn reorder(&self, ordering_rules_set: &OrderingRulesSet) -> Self {
        let mut reordered = self.clone();
        reordered.sort_by(|a, b| {
            if ordering_rules_set.contains(a, b) {
                Ordering::Less
            } else if ordering_rules_set.contains(b, a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        reordered
    }
}

impl Day {
    fn part1(input: Parsed) -> Result<Output> {
        Ok(input
            .sections
            .into_iter()
            .filter(|section| section.correctly_ordered(&input.ordering_rules))
            .map(|section| section.middle_page_number())
            .sum())
    }

    fn part2(input: Parsed) -> Result<Output> {
        let ordering_rules_set = OrderingRulesSet::from(&input.ordering_rules);
        Ok(input
            .sections
            .into_iter()
            .filter(|section| !section.correctly_ordered(&input.ordering_rules))
            .map(|section| section.reorder(&ordering_rules_set).middle_page_number())
            .sum())
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::input)(input)?.1)
    }

    fn input(s: &'static str) -> IResult<Input> {
        map(
            separated_pair(Self::ordering_rules, pair(newline, newline), Self::sections),
            |(ordering_rules, sections)| Input {
                ordering_rules,
                sections,
            },
        )(s)
    }

    fn ordering_rules(s: &'static str) -> IResult<Vec<OrderingRule>> {
        separated_list1(newline, Self::ordering_rule)(s)
    }

    fn ordering_rule(s: &'static str) -> IResult<OrderingRule> {
        map(separated_pair(u8, tag("|"), u8), |(before, after)| {
            OrderingRule {
                before: before as usize,
                after: after as usize,
            }
        })(s)
    }

    fn sections(s: &'static str) -> IResult<Vec<Section>> {
        separated_list1(newline, Self::pages)(s)
    }

    fn pages(s: &'static str) -> IResult<Vec<PageNumber>> {
        separated_list1(tag(","), map(u8, |page| page as usize))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 143);

    test_example!("example1", Part2, 123);
}
