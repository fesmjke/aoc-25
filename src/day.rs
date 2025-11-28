use std::{fmt::Display, io};

pub trait Day {
    type Input;

    fn parse(lines: impl Iterator<Item = io::Result<String>>) -> Self::Input;

    fn part_a(input: &Self::Input) -> impl Display;
    fn part_b(input: &Self::Input) -> impl Display;
}

pub struct Solution {
    pub part_a: String,
    pub part_b: String,
}

#[macro_export]
macro_rules! aoc_test {
    (
        $day_struct:ty,
        $input_file_name:expr,

        $part_a_example_answer:expr,
        $part_b_example_answer:expr,

        $part_a_answer:expr,
        $part_b_answer:expr
    ) => {
        #[cfg(test)]
        mod aoc_tests {
            use super::*;
            use crate::day::{Day, Solution};
            use crate::helpers::read::read_file;

            #[test]
            fn example() {
                let raw_path = format!("{}_example.txt", $input_file_name);
                let solution = solve(&raw_path);

                assert_eq!(solution.part_a, $part_a_example_answer.to_string());
                assert_eq!(solution.part_b, $part_b_example_answer.to_string());
            }

            #[test]
            fn puzzle() {
                let raw_path = format!("{}.txt", $input_file_name);
                let solution = solve(&raw_path);

                assert_eq!(solution.part_a, $part_a_answer.to_string());
                assert_eq!(solution.part_b, $part_b_answer.to_string());
            }

            fn solve(raw_path: &str) -> Solution {
                let path = format!("inputs/{}", raw_path);
                let lines = read_file(path).expect("Could not find input file!");

                let parsed = <$day_struct>::parse(lines);

                let part_a = <$day_struct>::part_a(&parsed);
                let part_b = <$day_struct>::part_b(&parsed);

                println!("╔════ Solving {} ({})", stringify!($day_struct), raw_path);
                println!("╠══ Part a: {} ", part_a);
                println!("╚══ Part b: {} ", part_b);

                Solution {
                    part_a: part_a.to_string(),
                    part_b: part_b.to_string(),
                }
            }
        }
    };
}
