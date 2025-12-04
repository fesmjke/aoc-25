use crate::{aoc_test, day::Day};

struct PrintingDepartment;

impl PrintingDepartment {
    fn check_adjacent(map: &Vec<Vec<char>>, check_y: usize, check_x: usize, limit: usize) -> bool {
        let mut adjacents = 0;
        let line_width = map[0].len();
        let line_height = map.len();

        for (offset_y, offset_x) in vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let new_position_x = check_x.checked_add_signed(offset_x);
            let new_position_y = check_y.checked_add_signed(offset_y);

            match (new_position_x, new_position_y) {
                (Some(x), Some(y)) => {
                    if x < line_height && y < line_width {
                        match map[y][x] {
                            '@' => {
                                adjacents += 1;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        adjacents < limit
    }
}

impl Day for PrintingDepartment {
    type Input = Vec<Vec<char>>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let mut count = 0;

        for (y, line) in input.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if PrintingDepartment::check_adjacent(&input, y, x, 4) && *ch == '@' {
                    count += 1;
                }
            }
        }
        count
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(PrintingDepartment, "day4", 13, 0, 1393, 0);
