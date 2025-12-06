use std::str::FromStr;

use crate::{aoc_test, day::Day, helpers::parse::parse_numbers_from_str};

struct TrashCompactor {}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(()),
        }
    }
}

impl Day for TrashCompactor {
    type Input = (Vec<Vec<u128>>, Vec<Operation>);

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        let mut nums = vec![];
        let mut operations = vec![];

        lines.filter_map(|line| line.ok()).for_each(|line| {
            if line.contains("*") || line.contains("+") {
                for ch in line
                    .split(char::is_whitespace)
                    .filter(|&line| line.len() >= 1)
                {
                    operations.push(Operation::from_str(ch).unwrap());
                }
            } else {
                nums.push(parse_numbers_from_str::<u128>(&line).unwrap());
            }
        });

        (nums, operations)
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let (nums, ops) = input;

        let nums_length = nums[0].len();
        let mut result: Vec<u128> = vec![0; nums_length];

        for (_, nums_line) in nums.iter().enumerate() {
            for (num_index, num) in nums_line.iter().enumerate() {
                match ops[num_index] {
                    Operation::Add => result[num_index] += num,
                    Operation::Mul => {
                        if result[num_index] == 0 {
                            result[num_index] = *num;
                        } else {
                            result[num_index] *= num
                        }
                    }
                }
            }
        }

        result.iter().sum::<u128>()
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(TrashCompactor, "day6", 4277556, 0, 0, 0);
