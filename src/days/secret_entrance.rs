use std::str::FromStr;

use crate::{aoc_test, day::Day};

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
pub struct ParseTurnError;

impl FromStr for Turn {
    type Err = ParseTurnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "L" {
            Ok(Self::Left)
        } else if s == "R" {
            Ok(Self::Right)
        } else {
            Err(ParseTurnError)
        }
    }
}

pub struct SecretEntrance;

impl Day for SecretEntrance {
    type Input = Vec<(Turn, i32)>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines
            .filter_map(|line| line.ok())
            .map(|line| {
                let (turn, clicks) = line.split_at(1);

                (
                    Turn::from_str(turn).expect("Unable to parse Turn"),
                    clicks.parse::<i32>().expect("Unable to parse clicks"),
                )
            })
            .collect()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let mut dial = 50;
        let mut count = 0;

        for &(turn, clicks) in input.iter() {
            match turn {
                Turn::Left => {
                    dial -= clicks;
                }
                Turn::Right => {
                    dial += clicks;
                }
            }

            let diff = dial % 100;

            dial = if diff < 0 { 100 + diff } else { diff };

            if dial == 0 {
                count += 1;
            }
        }

        count
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        let mut dial = 50;
        let mut count = 0;

        for &(turn, clicks) in input.iter() {
            let old_dial = dial;
            count += clicks / 100; // add crossed by N cycles

            match turn {
                Turn::Left => dial -= clicks % 100, // ensure that we are have clicks in range of 0 - 100
                Turn::Right => dial += clicks % 100,
            };

            if dial <= 0 || dial >= 100 {
                if old_dial != 0 {
                    // ensure that we are not at 0
                    count += 1
                }

                dial = dial.rem_euclid(100);
            }
        }

        count
    }
}

aoc_test!(SecretEntrance, "day1", 3, 6, 1145, 6561);
