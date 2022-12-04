use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 64929;
pub const PART_2: usize = 193697;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_01.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    calorie_counts: Vec<usize>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut calorie_counts = Vec::new();

        let mut count = 0;
        for line in data.lines() {
            if line.is_empty() {
                calorie_counts.push(count);
                count = 0;
                continue;
            }
            count += line.parse::<usize>().unwrap();
        }
        calorie_counts.push(count);

        calorie_counts.sort_unstable();
        calorie_counts.reverse();

        Self { calorie_counts }
    }

    pub fn part_1(&self) -> usize {
        self.calorie_counts.first().copied().unwrap()
    }

    pub fn part_2(&self) -> usize {
        self.calorie_counts.iter().take(3).sum()
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
                expected: 24000,
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
                expected: 45000,
            })
        }

        #[test]
        #[ignore = "Data directory does not exist in all locations"]
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
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            Input {
                calorie_counts: vec![24000, 11000, 10000, 6000, 4000],
            },
        )
    }
}
