use std::{
    collections::VecDeque,
    iter::{repeat_n, Flatten, RepeatN},
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::*;

day! {
    Output = usize,
    Parsed = DiskMap,
    bench_sample_size: 90,
}

#[derive(Debug, Clone)]
struct DiskMap {
    contents: Vec<DiskContent>,
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

impl<Idx> Index<Idx> for DiskMap
where
    Idx: SliceIndex<[DiskContent]>,
{
    type Output = <Idx as SliceIndex<[DiskContent]>>::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        self.contents.index(index)
    }
}

impl<Idx> IndexMut<Idx> for DiskMap
where
    Idx: SliceIndex<[DiskContent]>,
{
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.contents.index_mut(index)
    }
}

impl DiskMap {
    #[inline]
    fn len(&self) -> usize {
        self.contents.len()
    }
}

impl IntoIterator for DiskMap {
    type Item = DiskBlock;
    type IntoIter = Flatten<std::vec::IntoIter<DiskContent>>;

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
        let mut idx_file = disk_map.len() - 1;
        while idx_file > 0 {
            let mut idx_free_space = 1;
            while idx_free_space < idx_file {
                if disk_map[idx_free_space].length >= disk_map[idx_file].length {
                    disk_map[idx_free_space..=idx_file].rotate_right(2);

                    if idx_file + 1 < disk_map.len() {
                        disk_map[idx_file + 1].length +=
                            disk_map[idx_free_space].length + disk_map[idx_free_space + 1].length;
                    }
                    disk_map[idx_free_space + 2].length -= disk_map[idx_free_space + 1].length;
                    disk_map[idx_free_space].length = 0;

                    idx_file += 2;
                    break;
                }
                idx_free_space += 2;
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
