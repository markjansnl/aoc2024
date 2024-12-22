use std::{
    array,
    collections::{BTreeMap, BTreeSet},
    simd::prelude::*,
};

use crate::*;

day! {
    Output = u128,
    Parsed = [u32; 6_400],
    bench_sample_size: 10,
}

impl Day {
    fn part1(secrets: Parsed) -> Result<Output> {
        let prune = u32x64::splat(16_777_216);
        Ok(secrets
            .chunks(64)
            .map(|chunk| {
                let mut secret = u32x64::from_slice(chunk);
                for _ in 0..2000 {
                    secret = (secret ^ (secret << 6)) % prune;
                    secret = (secret ^ (secret >> 5)) % prune;
                    secret = (secret ^ (secret << 11)) % prune;
                }
                secret.reduce_sum() as u128
            })
            .sum())
    }

    fn part2(secrets: Parsed) -> Result<Output> {
        let prune = u32x64::splat(16_777_216);
        let ten = u32x64::splat(10);
        let hundred_thousand = u32x64::splat(100_000);
        let mut last_5_digits = u32x64::default();
        let mut sequences = BTreeMap::new();
        for chunk in secrets.chunks(64) {
            let mut secret = u32x64::from_slice(chunk);
            let mut sequences64 = vec![BTreeSet::new(); 64];
            for i in 0..2000 {
                secret = (secret ^ (secret << 6)) % prune;
                secret = (secret ^ (secret >> 5)) % prune;
                secret = (secret ^ (secret << 11)) % prune;
                let last = secret % ten;
                last_5_digits = (last_5_digits * ten + last) % hundred_thousand;
                if i >= 4 {
                    for j in 0..64 {
                        if last[j] > 0 {
                            let sequence = Self::last_5_digits_to_sequence(last_5_digits[j]);
                            if !sequences64[j].contains(&sequence) {
                                sequences64[j].insert(sequence);
                                *sequences.entry(sequence).or_default() += last[j] as u128;
                            }
                        }
                    }
                }
            }
        }

        Ok(sequences
            .into_iter()
            .max_by_key(|(_, bananas)| *bananas)
            .unwrap()
            .1)
    }

    fn last_5_digits_to_sequence(last_5_digits: u32) -> [i8; 4] {
        let digits = [
            (last_5_digits % 10) as i8,
            ((last_5_digits / 10) % 10) as i8,
            ((last_5_digits / 100) % 10) as i8,
            ((last_5_digits / 1_000) % 10) as i8,
            ((last_5_digits / 10_000) % 10) as i8,
        ];

        [
            digits[3] - digits[4],
            digits[2] - digits[3],
            digits[1] - digits[2],
            digits[0] - digits[1],
        ]
    }
}

impl Parser {
    fn parse(input: &'static str) -> Result<Parsed> {
        let mut iter = input.lines().map(|line| line.parse().unwrap());
        Ok(array::from_fn(|_| iter.next().unwrap_or_default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 37327623);

    test_example!("example2", Part2, 23);
}
