use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1343;
pub const PART_2: usize = 2193;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_06.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input<'b> {
    buffer: &'b str,
}

impl<'b> Input<'b> {
    pub fn from_data(data: &'b str) -> Self {
        Self { buffer: data }
    }

    pub fn part_1(&self) -> usize {
        self.marker(4)
    }

    pub fn part_2(&self) -> usize {
        self.marker(14)
    }

    fn marker(&self, marker_size: usize) -> usize {
        self.buffer
            .as_bytes()
            .windows(marker_size)
            .enumerate()
            .find(|(_, window)| {
                window
                    .iter()
                    .fold(0u32, |acc, b| acc | 1 << (b - b'a'))
                    .count_ones()
                    == marker_size as u32
            })
            .unwrap()
            .0
            + marker_size
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
            expected: Input<'c>,
        }

        #[test]
        fn example_1() {
            run(&Case {
                input: super::example_1().0,
                expected: super::example_1().1,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: super::example_2().0,
                expected: super::example_2().1,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                input: super::example_3().0,
                expected: super::example_3().1,
            })
        }

        #[test]
        fn example_4() {
            run(&Case {
                input: super::example_4().0,
                expected: super::example_4().1,
            })
        }

        #[test]
        fn example_5() {
            run(&Case {
                input: super::example_5().0,
                expected: super::example_5().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example_1() {
            run(&Case {
                data: super::example_1().1,
                expected: 7,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 5,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: super::example_3().1,
                expected: 6,
            })
        }

        #[test]
        fn example_4() {
            run(&Case {
                data: super::example_4().1,
                expected: 10,
            })
        }

        #[test]
        fn example_5() {
            run(&Case {
                data: super::example_5().1,
                expected: 11,
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

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example_1() {
            run(&Case {
                data: super::example_1().1,
                expected: 19,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 23,
            })
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: super::example_3().1,
                expected: 23,
            })
        }

        #[test]
        fn example_4() {
            run(&Case {
                data: super::example_4().1,
                expected: 29,
            })
        }

        #[test]
        fn example_5() {
            run(&Case {
                data: super::example_5().1,
                expected: 26,
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

    fn example_1() -> (&'static str, Input<'static>) {
        (
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            Input {
                buffer: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            },
        )
    }

    fn example_2() -> (&'static str, Input<'static>) {
        (
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            Input {
                buffer: "bvwbjplbgvbhsrlpgdmjqwftvncz",
            },
        )
    }

    fn example_3() -> (&'static str, Input<'static>) {
        (
            "nppdvjthqldpwncqszvftbrmjlhg",
            Input {
                buffer: "nppdvjthqldpwncqszvftbrmjlhg",
            },
        )
    }

    fn example_4() -> (&'static str, Input<'static>) {
        (
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            Input {
                buffer: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            },
        )
    }

    fn example_5() -> (&'static str, Input<'static>) {
        (
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            Input {
                buffer: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            },
        )
    }
}
