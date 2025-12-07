use crate::{aoc_test, day::Day};

struct Laboratories;

impl Day for Laboratories {
    type Input = Vec<String>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines.filter_map(|line| line.ok()).collect()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let start_position = input[0].len() / 2;

        let mut beam_indexes: Vec<(usize, usize)> = vec![(1, start_position)];

        let mut count = 0;

        for _ in input.iter().skip(1) {
            let mut splited_beams = vec![];
            let mut update_beams = vec![];

            beam_indexes.sort();
            beam_indexes.dedup();

            for beam in beam_indexes.iter() {
                let next = (beam.0 + 1, beam.1);

                if beam.0 + 1 == input.len() {
                    break;
                }

                match &input[next.0][beam.1..=beam.1] {
                    "." => {
                        if !beam_indexes.contains(&next) {
                            update_beams.push(next);
                        }
                    }
                    "^" => {
                        let splited_right = (next.0, next.1 + 1);
                        let splited_left = (next.0, next.1 - 1);

                        if !splited_beams.contains(&splited_left) {
                            splited_beams.push(splited_left);
                        }
                        if !splited_beams.contains(&splited_right) {
                            splited_beams.push(splited_right);
                        }

                        count += 1;
                    }
                    _ => {}
                }
            }

            update_beams.extend(splited_beams);

            beam_indexes = update_beams;
        }

        count
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(Laboratories, "day7", 21, 0, 1562, 0);
