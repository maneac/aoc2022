use std::{collections::HashSet, fs::read_to_string, path::Path};

pub const PART_1: usize = 592;
pub const PART_2: usize = 30367;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_14.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    max_y: usize,
    rocks: HashSet<(usize, usize)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut rocks = HashSet::new();
        let mut max_y = 0;
        for line in data.lines() {
            let points = line
                .split(" -> ")
                .map(|point| {
                    let (lhs, rhs) = point.split_once(',').unwrap();
                    (lhs.parse().unwrap(), rhs.parse().unwrap())
                })
                .collect::<Vec<(usize, usize)>>();
            for window in points.windows(2) {
                let (from, to) = (window[0], window[1]);
                for x in from.0.min(to.0)..=to.0.max(from.0) {
                    for y in from.1.min(to.1)..=to.1.max(from.1) {
                        max_y = max_y.max(y);
                        rocks.insert((x, y));
                    }
                }
            }
        }
        Self { max_y, rocks }
    }

    pub fn part_1(&self) -> usize {
        let sand_start = (500, 0);

        let mut filled = self.rocks.clone();

        let mut sand = sand_start;
        let mut count = 0;
        loop {
            if sand.1 > self.max_y {
                return count;
            }
            let straight_down = (sand.0, sand.1 + 1);
            if !filled.contains(&straight_down) {
                sand = straight_down;
                continue;
            }
            let left_down = (straight_down.0 - 1, straight_down.1);
            if !filled.contains(&left_down) {
                sand = left_down;
                continue;
            }
            let right_down = (straight_down.0 + 1, straight_down.1);
            if !filled.contains(&right_down) {
                sand = right_down;
                continue;
            }

            filled.insert(sand);
            sand = sand_start;
            count += 1;
        }
    }

    pub fn part_2(&self) -> usize {
        let floor = self.max_y + 2;
        let sand_start = (500, 0);

        let mut filled = self.rocks.clone();

        let mut sand = sand_start;
        let mut count = 0;
        loop {
            if sand.1 + 1 == floor {
                filled.insert(sand);
                sand = sand_start;
                count += 1;
                continue;
            }

            let straight_down = (sand.0, sand.1 + 1);
            if !filled.contains(&straight_down) {
                sand = straight_down;
                continue;
            }
            let left_down = (straight_down.0 - 1, straight_down.1);
            if !filled.contains(&left_down) {
                sand = left_down;
                continue;
            }
            let right_down = (straight_down.0 + 1, straight_down.1);
            if !filled.contains(&right_down) {
                sand = right_down;
                continue;
            }

            if sand == sand_start {
                return count + 1;
            }

            filled.insert(sand);
            sand = sand_start;
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example() {
            run(&Case {
                input: super::example().0,
                expected: super::example().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 24,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1())
        }
    }

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 93,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_2,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2())
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
            Input {
                max_y: 9,
                rocks: HashSet::from([
                    (498, 4),
                    (498, 5),
                    (498, 6),
                    (497, 6),
                    (496, 6),
                    (503, 4),
                    (502, 4),
                    (502, 5),
                    (502, 6),
                    (502, 7),
                    (502, 8),
                    (502, 9),
                    (501, 9),
                    (500, 9),
                    (499, 9),
                    (498, 9),
                    (497, 9),
                    (496, 9),
                    (495, 9),
                    (494, 9),
                ]),
            },
        )
    }
}
