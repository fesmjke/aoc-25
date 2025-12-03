use std::str::FromStr;

use crate::{aoc_test, day::Day};

struct Lobby;

struct Bank {
    batteries: Vec<u32>,
}

#[derive(Debug)]
struct BankParseError;

impl FromStr for Bank {
    type Err = BankParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

        Ok(Self { batteries: a })
    }
}

impl Day for Lobby {
    type Input = Vec<Bank>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines
            .filter_map(|line| line.ok())
            .map(|line| Bank::from_str(&line).unwrap())
            .collect()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let mut joltage = 0;
        for bank in input.iter() {
            let mut max_element_index = 0;
            let mut max_element = 0;

            for (index, &element) in bank.batteries.iter().enumerate() {
                if element > max_element {
                    max_element = element;
                    max_element_index = index;
                }
            }

            let left = &bank.batteries[..max_element_index];
            let right = &bank.batteries[max_element_index + 1..];

            if max_element_index == bank.batteries.len() - 1 {
                let max_left = left.iter().max().unwrap();

                joltage += format!("{}{}", max_left, max_element)
                    .parse::<u32>()
                    .unwrap();
            } else {
                let max_right = right.iter().max().unwrap();

                joltage += format!("{}{}", max_element, max_right)
                    .parse::<u32>()
                    .unwrap();
            }
        }

        joltage
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

#[test]
fn sample() {}

aoc_test!(Lobby, "day3", 357, 0, 16812, 0);
