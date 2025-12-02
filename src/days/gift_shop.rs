use std::{ops::Add, str::FromStr};

use crate::{aoc_test, day::Day};

#[derive(Debug, Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64,
}

#[derive(Debug)]
pub struct IdRangeParseError;

impl FromStr for IdRange {
    type Err = IdRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").ok_or(IdRangeParseError)?;

        let start = start.parse::<u64>().map_err(|_| IdRangeParseError)?;
        let end = end.parse::<u64>().map_err(|_| IdRangeParseError)?;

        Ok(Self {
            start: start,
            end: end,
        })
    }
}

struct GiftShop {}

impl GiftShop {
    pub fn equal_parts(left: &str, right: &str) -> bool {
        left == right
    }
}

impl Day for GiftShop {
    type Input = Vec<IdRange>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines
            .filter_map(|line| line.ok())
            .map(|line| {
                line.split(",")
                    .map(|x| IdRange::from_str(x).unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let mut count = 0;
        for range in input {
            // bruteforce solution
            let size_of_number = u64::ilog10(range.end) + 1;

            let ns = range.end - range.start;

            for i in 0..=ns {
                let left = &range.start.add(i).to_string()[0..size_of_number as usize / 2];
                let right = &range.start.add(i).to_string()[size_of_number as usize / 2..];

                if GiftShop::equal_parts(left, right) {
                    count += range.start + i;
                }
            }
        }
        count
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(GiftShop, "day2", 1227775554, 0, 0, 0);
