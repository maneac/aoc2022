use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 511;
pub const PART_2: usize = 821;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_04.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    assignments: Vec<[[u8; 2]; 2]>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut assignments = Vec::new();
        for line in data.lines() {
            let (lhs_str, rhs_str) = line.split_once(',').unwrap();

            let mut lhs_iter = lhs_str.split('-').map(|num| num.parse::<u8>().unwrap());
            let min_lhs = lhs_iter.next().unwrap();
            let max_lhs = lhs_iter.next().unwrap();
            let lhs = [min_lhs, max_lhs];

            let mut rhs_iter = rhs_str.split('-').map(|num| num.parse::<u8>().unwrap());
            let min_rhs = rhs_iter.next().unwrap();
            let max_rhs = rhs_iter.next().unwrap();
            let rhs = [min_rhs, max_rhs];

            let assignment = match min_lhs.cmp(&min_rhs) {
                std::cmp::Ordering::Less => [lhs, rhs],
                std::cmp::Ordering::Equal => {
                    if max_lhs > max_rhs {
                        [lhs, rhs]
                    } else {
                        [rhs, lhs]
                    }
                }
                std::cmp::Ordering::Greater => [rhs, lhs],
            };

            assignments.push(assignment);
        }

        Self { assignments }
    }

    pub fn part_1(&self) -> usize {
        self.assignments
            .iter()
            .filter(|[[min_lhs, max_lhs], [min_rhs, max_rhs]]| {
                min_lhs <= min_rhs && max_lhs >= max_rhs
            })
            .count()
    }

    pub fn part_2(&self) -> usize {
        self.assignments
            .iter()
            .filter(|[[_, max_lhs], [min_rhs, _]]| max_lhs >= min_rhs)
            .count()
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
                expected: 2,
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
                expected: 4,
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
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            Input {
                assignments: vec![
                    [[2, 4], [6, 8]],
                    [[2, 3], [4, 5]],
                    [[5, 7], [7, 9]],
                    [[2, 8], [3, 7]],
                    [[4, 6], [6, 6]],
                    [[2, 6], [4, 8]],
                ],
            },
        )
    }
}
