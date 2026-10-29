use crate::{aoc_test, day::Day};

struct MovieTheater {}

impl Day for MovieTheater {
    type Input = Vec<(u64, u64)>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines
            .filter_map(|line| line.ok())
            .map(|line| {
                let (l, r) = line.split_once(",").unwrap();

                return (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap());
            })
            .collect::<Vec<_>>()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let mut areas: Vec<u64> = vec![];
        for i in 0..input.len() - 1 {
            for j in i + 1..input.len() {
                let x = if input[i].0 == input[j].0 {
                    1
                } else {
                    if input[i].0 > input[j].0 {
                        (input[i].0 - input[j].0) + 1
                    } else {
                        (input[j].0 - input[i].0) + 1
                    }
                };
                let y = if input[i].1 == input[j].1 {
                    1
                } else {
                    if input[i].1 > input[j].1 {
                        (input[i].1 - input[j].1) + 1
                    } else {
                        (input[j].1 - input[i].1) + 1
                    }
                };
                let area = x * y;

                areas.push(area);
            }
        }

        areas.into_iter().max().unwrap()
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(MovieTheater, "day9", 50, 0, 4763509452 as u64, 0);
