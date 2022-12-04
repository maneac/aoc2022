use std::{cmp::Ordering, fs::read_to_string, path::Path};

pub const PART_1: usize = 15572;
pub const PART_2: usize = 16098;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_02.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    strategies: Vec<[Shape; 2]>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut strategies = Vec::new();
        for line in data.lines() {
            let parts = line
                .split_whitespace()
                .map(Shape::from)
                .collect::<Vec<Shape>>();
            assert_eq!(parts.len(), 2);
            strategies.push([parts[0], parts[1]]);
        }
        Self { strategies }
    }

    pub fn part_1(&self) -> usize {
        self.strategies.iter().fold(0, |acc, &[opponent, me]| {
            acc + (me as usize)
                + match me.cmp(&opponent) {
                    Ordering::Less => Outcome::Lose,
                    Ordering::Equal => Outcome::Draw,
                    Ordering::Greater => Outcome::Win,
                } as usize
        })
    }

    pub fn part_2(&self) -> usize {
        self.strategies.iter().fold(0, |acc, &[opponent, target]| {
            acc + match target {
                Shape::Rock => {
                    Outcome::Lose as usize
                        + match opponent {
                            Shape::Rock => Shape::Scissors,
                            Shape::Paper => Shape::Rock,
                            Shape::Scissors => Shape::Paper,
                        } as usize
                }
                Shape::Paper => Outcome::Draw as usize + opponent as usize,
                Shape::Scissors => {
                    Outcome::Win as usize
                        + match opponent {
                            Shape::Rock => Shape::Paper,
                            Shape::Paper => Shape::Scissors,
                            Shape::Scissors => Shape::Rock,
                        } as usize
                }
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Some(Ordering::Equal),
                Shape::Paper => Some(Ordering::Less),
                Shape::Scissors => Some(Ordering::Greater),
            },
            Shape::Paper => match other {
                Shape::Rock => Some(Ordering::Greater),
                Shape::Paper => Some(Ordering::Equal),
                Shape::Scissors => Some(Ordering::Less),
            },
            Shape::Scissors => match other {
                Shape::Rock => Some(Ordering::Less),
                Shape::Paper => Some(Ordering::Greater),
                Shape::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("unknown shape: {value}"),
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
        fn rock_rock() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Rock, Shape::Rock]],
                },
                expected: Outcome::Draw as usize + Shape::Rock as usize,
            });
        }

        #[test]
        fn rock_paper() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Rock, Shape::Paper]],
                },
                expected: Outcome::Win as usize + Shape::Paper as usize,
            });
        }

        #[test]
        fn rock_scissors() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Rock, Shape::Scissors]],
                },
                expected: Outcome::Lose as usize + Shape::Scissors as usize,
            });
        }

        #[test]
        fn papoer_rock() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Paper, Shape::Rock]],
                },
                expected: Outcome::Lose as usize + Shape::Rock as usize,
            });
        }

        #[test]
        fn paper_paper() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Paper, Shape::Paper]],
                },
                expected: Outcome::Draw as usize + Shape::Paper as usize,
            });
        }

        #[test]
        fn paper_scissors() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Paper, Shape::Scissors]],
                },
                expected: Outcome::Win as usize + Shape::Scissors as usize,
            });
        }

        #[test]
        fn scissors_rock() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Scissors, Shape::Rock]],
                },
                expected: Outcome::Win as usize + Shape::Rock as usize,
            });
        }

        #[test]
        fn scissors_paper() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Scissors, Shape::Paper]],
                },
                expected: Outcome::Lose as usize + Shape::Paper as usize,
            });
        }

        #[test]
        fn scissors_scissors() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Scissors, Shape::Scissors]],
                },
                expected: Outcome::Draw as usize + Shape::Scissors as usize,
            });
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 15,
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
        fn rock_rock() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Rock, Shape::Rock]],
                },
                expected: Outcome::Lose as usize + Shape::Scissors as usize,
            });
        }

        #[test]
        fn rock_paper() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Rock, Shape::Paper]],
                },
                expected: Outcome::Draw as usize + Shape::Rock as usize,
            });
        }

        #[test]
        fn rock_scissors() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Rock, Shape::Scissors]],
                },
                expected: Outcome::Win as usize + Shape::Paper as usize,
            });
        }

        #[test]
        fn papoer_rock() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Paper, Shape::Rock]],
                },
                expected: Outcome::Lose as usize + Shape::Rock as usize,
            });
        }

        #[test]
        fn paper_paper() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Paper, Shape::Paper]],
                },
                expected: Outcome::Draw as usize + Shape::Paper as usize,
            });
        }

        #[test]
        fn paper_scissors() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Paper, Shape::Scissors]],
                },
                expected: Outcome::Win as usize + Shape::Scissors as usize,
            });
        }

        #[test]
        fn scissors_rock() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Scissors, Shape::Rock]],
                },
                expected: Outcome::Lose as usize + Shape::Paper as usize,
            });
        }

        #[test]
        fn scissors_paper() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Scissors, Shape::Paper]],
                },
                expected: Outcome::Draw as usize + Shape::Scissors as usize,
            });
        }

        #[test]
        fn scissors_scissors() {
            run(&Case {
                data: Input {
                    strategies: vec![[Shape::Scissors, Shape::Scissors]],
                },
                expected: Outcome::Win as usize + Shape::Rock as usize,
            });
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 12,
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
            "A Y
B X
C Z",
            Input {
                strategies: vec![
                    [Shape::Rock, Shape::Paper],
                    [Shape::Paper, Shape::Rock],
                    [Shape::Scissors, Shape::Scissors],
                ],
            },
        )
    }
}
