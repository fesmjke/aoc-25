use std::{collections::VecDeque, str::FromStr};

use crate::{aoc_test, day::Day};

struct Lobby;

impl Lobby {}

struct Bank {
    batteries: Vec<u64>,
}

#[derive(Debug)]
struct BankParseError;

impl FromStr for Bank {
    type Err = BankParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();

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
        let mut joltage = 0;
        const BANK_SIZE: usize = 12;
        for bank in input.iter() {
            let length = bank.batteries.len();

            let mut next = 0;
            let mut local_joltage = 0;

            for n in (0..BANK_SIZE).rev() {
                let mut max: u64 = 0;

                for idx in next..(length - n) {
                    if bank.batteries[idx] > max {
                        max = bank.batteries[idx];
                        next = idx + 1;
                    }
                }

                local_joltage = local_joltage * 10 + max;
            }

            joltage += local_joltage;
        }

        joltage
    }
}

aoc_test!(
    Lobby,
    "day3",
    357,
    3121910778619 as u64,
    16812,
    166345822896410 as u64
);
