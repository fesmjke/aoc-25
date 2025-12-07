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
    type Input = (Vec<Vec<u128>>, Vec<Operation>, Vec<String>);

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        let mut nums = vec![];
        let mut operations = vec![];
        let mut raw = vec![];

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

            raw.push(line);
        });

        (nums, operations, raw)
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let (nums, ops, _) = input;

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
        let (_, _, raw) = input;

        let ops_line = &raw[raw.len() - 1];
        let rows = raw.len() - 1; // total rows - operation line
        let mut result = 0;
        let mut current_op = Operation::Add;
        let mut local_result = vec![];
        let ops_line_len = ops_line.len();

        for (i, _) in ops_line.chars().enumerate() {
            let mut num = String::new();
            let local_op = &ops_line[i..=i];

            match Operation::from_str(local_op) {
                Ok(op) => {
                    match current_op {
                        Operation::Add => {
                            result += local_result.iter().sum::<u128>();
                        }
                        Operation::Mul => {
                            result += local_result.iter().fold(1, |acc, e| acc * e);
                        }
                    }

                    local_result.clear();
                    current_op = op;
                }
                Err(_) => {}
            };

            for j in 0..rows {
                num.push_str(&raw[j][i..=i]);
            }

            match current_op {
                Operation::Add => {
                    local_result.push(num.trim().parse::<u128>().unwrap_or(0));
                }
                Operation::Mul => {
                    local_result.push(num.trim().parse::<u128>().unwrap_or(1));
                }
            }
        }

        match current_op {
            Operation::Add => {
                result += local_result.iter().sum::<u128>();
            }
            Operation::Mul => {
                result += local_result.iter().fold(1, |acc, e| acc * e);
            }
        }

        result
    }
}

aoc_test!(
    TrashCompactor,
    "day6",
    4277556,
    3263827,
    6343365546996 as u128,
    11136895955912 as u128
);
