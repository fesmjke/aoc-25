use crate::{aoc_test, day::Day};

struct Cafeteria;

impl Day for Cafeteria {
    type Input = (Vec<(u128, u128)>, Vec<u128>);

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        let mut ranges = Vec::<(u128, u128)>::new();
        let mut ids = Vec::<u128>::new();
        lines.filter_map(|line| line.ok()).for_each(|line| {
            if !line.is_empty() {
                if line.contains("-") {
                    let (range_start_str, range_end_str) = line.split_once("-").unwrap();

                    ranges.push((
                        range_start_str.parse().unwrap(),
                        range_end_str.parse().unwrap(),
                    ))
                } else {
                    ids.push(line.parse().unwrap())
                }
            }
        });

        (ranges, ids)
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let (ranges, ids) = input;
        let mut count = 0;

        'ids_loop: for id in ids.iter() {
            for (range_start, range_end) in ranges.iter() {
                let range = range_start..range_end;

                if range.contains(&id) {
                    count += 1;
                    continue 'ids_loop;
                }
            }
        }

        count
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(Cafeteria, "day5", 3, 0, 726, 0);
