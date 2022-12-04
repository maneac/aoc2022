use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 8039;
pub const PART_2: usize = 2510;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_03.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    rucksacks: Vec<Rucksack>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        Self {
            rucksacks: data.lines().map(Rucksack::from_contents).collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.rucksacks
            .iter()
            .map(|r| (r.compartments[0] & r.compartments[1]).trailing_zeros() as usize)
            .sum()
    }

    pub fn part_2(&self) -> usize {
        self.rucksacks
            .chunks_exact(3)
            .map(|chunk| {
                ((chunk[0].compartments[0] | chunk[0].compartments[1])
                    & (chunk[1].compartments[0] | chunk[1].compartments[1])
                    & (chunk[2].compartments[0] | chunk[2].compartments[1]))
                    .trailing_zeros() as usize
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rucksack {
    compartments: [u64; 2],
}

impl Rucksack {
    fn from_contents(contents: &str) -> Self {
        let parts = contents.split_at(contents.len() / 2);
        Self {
            compartments: [
                contents_to_priority_bitset(parts.0),
                contents_to_priority_bitset(parts.1),
            ],
        }
    }
}

fn contents_to_priority_bitset(contents: &str) -> u64 {
    contents.bytes().fold(0, |acc, chr| {
        let shift = if (b'a'..=b'z').contains(&chr) {
            chr - b'a' + 1
        } else {
            chr - b'A' + 27
        };
        acc | (1 << shift)
    })
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
                expected: 157,
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
                expected: 70,
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
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            Input {
                rucksacks: vec![
                    Rucksack {
                        compartments: [
                            contents_to_priority_bitset("vJrwpWtwJgWr"),
                            contents_to_priority_bitset("hcsFMMfFFhFp"),
                        ],
                    },
                    Rucksack {
                        compartments: [
                            contents_to_priority_bitset("jqHRNqRjqzjGDLGL"),
                            contents_to_priority_bitset("rsFMfFZSrLrFZsSL"),
                        ],
                    },
                    Rucksack {
                        compartments: [
                            contents_to_priority_bitset("PmmdzqPrV"),
                            contents_to_priority_bitset("vPwwTWBwg"),
                        ],
                    },
                    Rucksack {
                        compartments: [
                            contents_to_priority_bitset("wMqvLMZHhHMvwLH"),
                            contents_to_priority_bitset("jbvcjnnSBnvTQFn"),
                        ],
                    },
                    Rucksack {
                        compartments: [
                            contents_to_priority_bitset("ttgJtRGJ"),
                            contents_to_priority_bitset("QctTZtZT"),
                        ],
                    },
                    Rucksack {
                        compartments: [
                            contents_to_priority_bitset("CrZsJsPPZsGz"),
                            contents_to_priority_bitset("wwsLwLmpwMDw"),
                        ],
                    },
                ],
            },
        )
    }
}
