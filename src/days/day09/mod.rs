use std::{
    collections::VecDeque,
    iter::{repeat_n, Flatten, RepeatN},
};

use crate::*;

day! {
    Output = usize,
    Parsed = DiskMap,
    bench_sample_size: 70,
}

#[derive(Debug, Clone)]
struct DiskMap {
    contents: VecDeque<DiskContent>,
}

#[derive(Debug, Clone, Copy)]
struct DiskContent {
    length: usize,
    content: DiskBlock,
}

#[derive(Debug, Clone, Copy)]
enum DiskBlock {
    File { id: usize },
    FreeSpace,
}

impl IntoIterator for DiskMap {
    type Item = DiskBlock;
    type IntoIter = Flatten<std::collections::vec_deque::IntoIter<DiskContent>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter().flatten()
    }
}

impl IntoIterator for DiskContent {
    type Item = DiskBlock;
    type IntoIter = RepeatN<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        repeat_n(self.content, self.length)
    }
}

impl Day {
    #[inline]
    fn part1(disk_map: Parsed) -> Result<Output> {
        let mut disk_blocks = VecDeque::from_iter(disk_map);
        let mut checksum = 0;
        let mut idx = 0;
        while let Some(disk_block) = disk_blocks.pop_front() {
            match disk_block {
                DiskBlock::File { id } => {
                    checksum += idx * id;
                    idx += 1
                }
                DiskBlock::FreeSpace => loop {
                    match disk_blocks.pop_back() {
                        None => break,
                        Some(DiskBlock::File { id }) => {
                            checksum += idx * id;
                            idx += 1;
                            break;
                        }
                        Some(DiskBlock::FreeSpace) => continue,
                    }
                },
            }
        }

        Ok(checksum)
    }

    #[inline]
    fn part2(mut disk_map: Parsed) -> Result<Output> {
        let mut idx_file = disk_map.contents.len() - 1;
        while idx_file > 0 {
            let mut idx_gap = 1;
            while idx_gap < idx_file {
                if disk_map.contents[idx_gap].length >= disk_map.contents[idx_file].length {
                    let file = disk_map.contents.remove(idx_file).unwrap();
                    if idx_file > 0 {
                        disk_map.contents[idx_file - 1].length += file.length;
                    }
                    if idx_file < disk_map.contents.len() {
                        if idx_file > 0 {
                            disk_map.contents[idx_file - 1].length +=
                                disk_map.contents[idx_file].length;
                        }
                        disk_map.contents.remove(idx_file);
                    }
                    disk_map.contents[idx_gap].length -= file.length;
                    disk_map.contents.insert(idx_gap, file);
                    disk_map.contents.insert(
                        idx_gap,
                        DiskContent {
                            length: 0,
                            content: DiskBlock::FreeSpace,
                        },
                    );
                    idx_file += 2;
                    break;
                }
                idx_gap += 2;
            }
            idx_file -= 2;
        }

        Ok(disk_map
            .into_iter()
            .enumerate()
            .map(|(idx, disk_block)| match disk_block {
                DiskBlock::File { id } => idx * id,
                DiskBlock::FreeSpace => 0,
            })
            .sum())
    }
}

impl Parser {
    #[inline]
    fn parse(input: &'static str) -> Result<Parsed> {
        Ok(DiskMap {
            contents: input
                .bytes()
                .enumerate()
                .map(|(idx, b)| DiskContent {
                    length: b as usize - 48,
                    content: if idx % 2 == 0 {
                        DiskBlock::File { id: idx / 2 }
                    } else {
                        DiskBlock::FreeSpace
                    },
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    run!(Part1);

    run!(Part2);

    test_example!("example1", Part1, 1928);

    test_example!("example1", Part2, 2858);
}
