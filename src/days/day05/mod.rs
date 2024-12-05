use std::{cmp::Ordering, collections::HashSet};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct OrderingRule {
    before: PageNumber,
    after: PageNumber,
}

struct PageMap {
    indices: [Option<usize>; 100],
}

impl From<&Section> for PageMap {
    #[inline]
    fn from(section: &Section) -> Self {
        let mut indices = [None; 100];
        for (idx, &page) in section.iter().enumerate() {
            indices[page] = Some(idx);
        }
        Self { indices }
    }
}

trait MiddlePageNumber {
    fn middle_page_number(&self) -> PageNumber;
}

impl MiddlePageNumber for Vec<PageNumber> {
    #[inline]
    fn middle_page_number(&self) -> PageNumber {
        self[(self.len() - 1) / 2]
    }
}

trait Reorder {
    fn reorder(&self, ordering_rules: &HashSet<OrderingRule>) -> Self;
}

impl Reorder for Section {
    #[inline]
    fn reorder(&self, ordering_rules_set: &HashSet<OrderingRule>) -> Self {
        let mut reordered = self.clone();
        reordered.sort_by(|a, b| {
            if ordering_rules_set.contains(&OrderingRule {
                before: *a,
                after: *b,
            }) {
                Ordering::Less
            } else if ordering_rules_set.contains(&OrderingRule {
                before: *b,
                after: *a,
            }) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        reordered
    }
}

impl Input {
    #[inline]
    fn sum_middle_page_numbers(&self, part: Part) -> Output {
        let ordering_rules_set = match part {
            Part1 => HashSet::new(),
            Part2 => self.ordering_rules.iter().copied().collect(),
        };

        self.sections
            .iter()
            .filter_map(|section| {
                let page_map = PageMap::from(section);
                let correctly_orderd =
                    self.ordering_rules
                        .iter()
                        .all(|OrderingRule { before, after }| {
                            match (page_map.indices[*before], page_map.indices[*after]) {
                                (Some(idx_before), Some(idx_after)) => idx_before < idx_after,
                                _ => true,
                            }
                        });

                match part {
                    Part1 => correctly_orderd.then_some(section.middle_page_number()),
                    Part2 => (!correctly_orderd)
                        .then_some(section.reorder(&ordering_rules_set).middle_page_number()),
                }
            })
            .sum()
    }
}

impl Day {
    #[inline]
    fn part1(input: Parsed) -> Result<Output> {
        Ok(input.sum_middle_page_numbers(Part1))
    }

    #[inline]
    fn part2(input: Parsed) -> Result<Output> {
        Ok(input.sum_middle_page_numbers(Part2))
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(all_consuming(Self::input)(input)?.1)
    }

    #[inline]
    fn input(s: &'static str) -> IResult<Input> {
        map(
            separated_pair(Self::ordering_rules, pair(newline, newline), Self::sections),
            |(ordering_rules, sections)| Input {
                ordering_rules,
                sections,
            },
        )(s)
    }

    #[inline]
    fn ordering_rules(s: &'static str) -> IResult<Vec<OrderingRule>> {
        separated_list1(newline, Self::ordering_rule)(s)
    }

    #[inline]
    fn ordering_rule(s: &'static str) -> IResult<OrderingRule> {
        map(separated_pair(u8, tag("|"), u8), |(before, after)| {
            OrderingRule {
                before: before as usize,
                after: after as usize,
            }
        })(s)
    }

    #[inline]
    fn sections(s: &'static str) -> IResult<Vec<Section>> {
        separated_list1(newline, Self::pages)(s)
    }

    #[inline]
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
